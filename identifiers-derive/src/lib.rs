use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident, Visibility};

fn require_newtype(input: &DeriveInput) -> Result<(), Error> {
    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => Ok(()),
            _ => Err(Error::new_spanned(
                input,
                "expected a newtype struct with exactly one unnamed field",
            )),
        },
        _ => Err(Error::new_spanned(input, "expected a struct")),
    }
}

/// Rejects `pub` fields when a validated `#[allowed_values]` mode is in use.
///
/// A `pub` inner field lets callers bypass the conversion impl and construct
/// the type directly with an unchecked value, undermining the invariant.
fn require_private_field(input: &DeriveInput) -> Result<(), Error> {
    if let Data::Struct(s) = &input.data {
        if let Fields::Unnamed(f) = &s.fields {
            if let Some(field) = f.unnamed.first() {
                if matches!(field.vis, Visibility::Public(_)) {
                    return Err(Error::new_spanned(
                        &field.vis,
                        "field must be private to preserve the non-empty/non-blank invariant; \
                         remove `pub` from the field",
                    ));
                }
            }
        }
    }
    Ok(())
}

enum AllowedValues {
    All,
    NonEmpty,
    NonBlank,
}

/// Parses `#[allowed_values(...)]` for `StringIdentifier`.
/// Valid options: `all`, `non_empty`, `non_blank`.
fn parse_allowed_values_string(input: &DeriveInput) -> Result<Option<AllowedValues>, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("allowed_values") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "all" => Ok(Some(AllowedValues::All)),
                "non_empty" => Ok(Some(AllowedValues::NonEmpty)),
                "non_blank" => Ok(Some(AllowedValues::NonBlank)),
                _ => Err(Error::new_spanned(
                    &mode,
                    "expected `all`, `non_empty`, or `non_blank`",
                )),
            };
        }
    }
    Ok(None)
}

/// Parses `#[allowed_values(all)]` for UUID, URI, and integer derives.
/// Returns `true` if present, `false` if absent.
fn parse_allowed_values_simple(input: &DeriveInput) -> Result<bool, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("allowed_values") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "all" => Ok(true),
                _ => Err(Error::new_spanned(&mode, "expected `all`")),
            };
        }
    }
    Ok(false)
}

/// Derives [`UuidIdentifier`](identifiers_uuid::UuidIdentifier) for a newtype wrapper around
/// [`uuid::Uuid`].
///
/// Always implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, and
/// `UuidIdentifier`.
///
/// Add `#[allowed_values(all)]` to also derive `From<Uuid>`.
#[proc_macro_derive(UuidIdentifier, attributes(allowed_values))]
pub fn derive_uuid_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uuid_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uuid_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let all = parse_allowed_values_simple(input)?;

    let base_impls = quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name)).field(&self.0).finish()
            }
        }

        impl ::std::clone::Clone for #name {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::marker::Copy for #name {}

        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl ::std::cmp::Eq for #name {}

        impl ::std::hash::Hash for #name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };

    let from_impl = if all {
        quote! {
            impl ::std::convert::From<::identifiers_uuid::__private::uuid::Uuid> for #name {
                fn from(uuid: ::identifiers_uuid::__private::uuid::Uuid) -> Self {
                    Self(uuid)
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #base_impls

        #from_impl

        impl ::identifiers_uuid::UuidIdentifier for #name {
            type Error = ::std::convert::Infallible;

            fn validate(
                _: &::identifiers_uuid::__private::uuid::Uuid,
            ) -> ::std::result::Result<(), ::std::convert::Infallible> {
                ::std::result::Result::Ok(())
            }

            fn new() -> Self {
                Self(::identifiers_uuid::__private::uuid::Uuid::new_v4())
            }

            fn as_uuid(&self) -> ::identifiers_uuid::__private::uuid::Uuid {
                self.0
            }
        }
    })
}

/// Derives [`StringIdentifier`](identifiers::StringIdentifier) for a newtype wrapper around
/// [`String`].
///
/// Always implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `AsRef<str>`, and
/// `StringIdentifier`.
///
/// Add an `#[allowed_values(...)]` attribute to also derive a conversion impl:
///
/// - `#[allowed_values(all)]` — derives `From<String>`
/// - `#[allowed_values(non_empty)]` — derives `TryFrom<String>`, rejects empty strings
/// - `#[allowed_values(non_blank)]` — derives `TryFrom<String>`, rejects blank strings
#[proc_macro_derive(StringIdentifier, attributes(allowed_values))]
pub fn derive_string_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_string_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_string_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let mode = parse_allowed_values_string(input)?;

    let base_impls = quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name)).field(&self.0).finish()
            }
        }

        impl ::std::clone::Clone for #name {
            fn clone(&self) -> Self { Self(self.0.clone()) }
        }

        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl ::std::cmp::Eq for #name {}

        impl ::std::hash::Hash for #name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl ::std::convert::AsRef<str> for #name {
            fn as_ref(&self) -> &str { &self.0 }
        }
    };

    if matches!(
        mode,
        Some(AllowedValues::NonEmpty) | Some(AllowedValues::NonBlank)
    ) {
        require_private_field(input)?;
    }

    let (error_type, validate_body) = match &mode {
        None | Some(AllowedValues::All) => (
            quote! { ::std::convert::Infallible },
            quote! { ::std::result::Result::Ok(()) },
        ),
        Some(AllowedValues::NonEmpty) => (
            quote! { ::identifiers::EmptyError },
            quote! {
                if s.is_empty() {
                    ::std::result::Result::Err(::identifiers::EmptyError)
                } else {
                    ::std::result::Result::Ok(())
                }
            },
        ),
        Some(AllowedValues::NonBlank) => (
            quote! { ::identifiers::BlankError },
            quote! {
                if s.trim().is_empty() {
                    ::std::result::Result::Err(::identifiers::BlankError)
                } else {
                    ::std::result::Result::Ok(())
                }
            },
        ),
    };

    let string_identifier_impl = quote! {
        impl ::identifiers::StringIdentifier for #name {
            type Error = #error_type;

            fn validate(s: &str) -> ::std::result::Result<(), Self::Error> {
                #validate_body
            }
        }
    };

    let conversion_impl = match &mode {
        None => quote! {},
        Some(AllowedValues::All) => quote! {
            impl ::std::convert::From<::std::string::String> for #name {
                fn from(s: ::std::string::String) -> Self {
                    Self(s)
                }
            }
        },
        Some(AllowedValues::NonEmpty) | Some(AllowedValues::NonBlank) => quote! {
            impl ::std::convert::TryFrom<::std::string::String> for #name {
                type Error = #error_type;

                fn try_from(
                    s: ::std::string::String,
                ) -> ::std::result::Result<Self, Self::Error> {
                    <Self as ::identifiers::StringIdentifier>::validate(&s)?;
                    ::std::result::Result::Ok(Self(s))
                }
            }
        },
    };

    Ok(quote! {
        #base_impls
        #string_identifier_impl
        #conversion_impl
    })
}

/// Derives [`UriIdentifier`](identifiers_uri::UriIdentifier) for a newtype wrapper around
/// [`fluent_uri::Uri<String>`].
///
/// Always implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, and `UriIdentifier`.
///
/// Add `#[allowed_values(all)]` to also derive `From<Uri<String>>`.
#[proc_macro_derive(UriIdentifier, attributes(allowed_values))]
pub fn derive_uri_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uri_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uri_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let all = parse_allowed_values_simple(input)?;

    let base_impls = quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name)).field(&self.0.as_str()).finish()
            }
        }

        impl ::std::clone::Clone for #name {
            fn clone(&self) -> Self { Self(self.0.clone()) }
        }

        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl ::std::cmp::Eq for #name {}

        impl ::std::hash::Hash for #name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };

    let from_impl = if all {
        quote! {
            impl ::std::convert::From<::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>> for #name {
                fn from(
                    uri: ::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>,
                ) -> Self {
                    Self(uri)
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #base_impls

        #from_impl

        impl ::identifiers_uri::UriIdentifier for #name {
            type Error = ::std::convert::Infallible;

            fn validate(
                _: &::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>,
            ) -> ::std::result::Result<(), ::std::convert::Infallible> {
                ::std::result::Result::Ok(())
            }

            fn as_uri(
                &self,
            ) -> &::identifiers_uri::__private::fluent_uri::Uri<::std::string::String> {
                &self.0
            }
        }
    })
}

/// Derives [`IntegerIdentifier`](identifiers::IntegerIdentifier) for a newtype wrapper around
/// `u64`.
///
/// Always implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`,
/// `Ord`, and `IntegerIdentifier`.
///
/// Add `#[allowed_values(all)]` to also derive `From<u64>`.
#[proc_macro_derive(IntegerIdentifier, attributes(allowed_values))]
pub fn derive_integer_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_integer_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_integer_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let all = parse_allowed_values_simple(input)?;

    let base_impls = quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!(#name)).field(&self.0).finish()
            }
        }

        impl ::std::clone::Clone for #name {
            fn clone(&self) -> Self { *self }
        }

        impl ::std::marker::Copy for #name {}

        impl ::std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl ::std::cmp::Eq for #name {}

        impl ::std::hash::Hash for #name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl ::std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                ::std::option::Option::Some(self.cmp(other))
            }
        }

        impl ::std::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    };

    let from_impl = if all {
        quote! {
            impl ::std::convert::From<u64> for #name {
                fn from(n: u64) -> Self {
                    Self(n)
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #base_impls

        #from_impl

        impl ::identifiers::IntegerIdentifier for #name {
            type Error = ::std::convert::Infallible;

            fn validate(_: u64) -> ::std::result::Result<(), ::std::convert::Infallible> {
                ::std::result::Result::Ok(())
            }

            fn zero() -> Self {
                Self(0)
            }

            fn as_u64(&self) -> u64 {
                self.0
            }
        }
    })
}
