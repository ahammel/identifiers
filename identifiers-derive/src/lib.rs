use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident};

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

enum ValidateMode {
    NonEmpty,
    NonBlank,
    Any,
}

/// Returns `true` if `#[validate(custom)]` is present, `false` if absent.
/// Used by UUID, URI, and integer derives where the only option is opting in to custom
/// validation; the default is infallible acceptance of all values.
fn parse_custom_validate_attr(input: &DeriveInput) -> Result<bool, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("validate") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "all" => Ok(false),
                "custom" => Ok(true),
                _ => Err(Error::new_spanned(&mode, "expected `all` or `custom`")),
            };
        }
    }
    Ok(false)
}

fn parse_validate_attr(input: &DeriveInput) -> Result<Option<ValidateMode>, Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("validate") {
            let mode: Ident = attr.parse_args()?;
            return match mode.to_string().as_str() {
                "non_empty" => Ok(Some(ValidateMode::NonEmpty)),
                "non_blank" => Ok(Some(ValidateMode::NonBlank)),
                "any" => Ok(Some(ValidateMode::Any)),
                "custom" => Ok(None),
                _ => Err(Error::new_spanned(
                    &mode,
                    "expected non_empty, non_blank, any, or custom",
                )),
            };
        }
    }
    Ok(None)
}

/// Derives [`UuidIdentifier`](identifiers_uuid::UuidIdentifier) for a newtype wrapper around
/// [`uuid::Uuid`].
///
/// Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `TryFrom<uuid::Uuid>`,
/// and `UuidIdentifier`.
///
/// By default, generates a no-op `validate` (accepts all UUIDs); `#[validate(all)]` is an
/// explicit alias for the same. Add `#[validate(custom)]` to suppress the trait impl and
/// supply your own.
#[proc_macro_derive(UuidIdentifier, attributes(validate))]
pub fn derive_uuid_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uuid_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uuid_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let custom = parse_custom_validate_attr(input)?;

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

        impl ::std::convert::TryFrom<::identifiers_uuid::__private::uuid::Uuid> for #name {
            type Error = <Self as ::identifiers_uuid::UuidIdentifier>::Error;

            fn try_from(
                uuid: ::identifiers_uuid::__private::uuid::Uuid,
            ) -> ::std::result::Result<Self, Self::Error> {
                <Self as ::identifiers_uuid::UuidIdentifier>::validate(&uuid)?;
                ::std::result::Result::Ok(Self(uuid))
            }
        }
    };

    if custom {
        return Ok(base_impls);
    }

    Ok(quote! {
        #base_impls

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
/// Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `TryFrom<String>`,
/// and `StringIdentifier`.
///
/// Requires a `#[validate(...)]` attribute specifying the validation strategy, or a manual
/// `impl StringIdentifier` providing `validate` and `as_str`:
///
/// - `#[validate(non_empty)]` — rejects empty strings
/// - `#[validate(non_blank)]` — rejects blank strings (empty or all whitespace)
/// - `#[validate(any)]` — accepts all strings, including empty and blank
/// - `#[validate(custom)]` — same as omitting the attribute; user implements `validate` and `as_str`
#[proc_macro_derive(StringIdentifier, attributes(validate))]
pub fn derive_string_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_string_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_string_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let mode = parse_validate_attr(input)?;

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

    let Some(mode) = mode else {
        // No #[validate(...)]: generate boilerplate and a TryFrom that delegates to the
        // caller's StringIdentifier impl. The caller must implement validate and as_str.
        return Ok(quote! {
            #base_impls

            impl ::std::convert::TryFrom<::std::string::String> for #name {
                type Error = <Self as ::identifiers::StringIdentifier>::Error;

                fn try_from(s: ::std::string::String) -> ::std::result::Result<Self, Self::Error> {
                    <Self as ::identifiers::StringIdentifier>::validate(&s)?;
                    ::std::result::Result::Ok(Self(s))
                }
            }
        });
    };

    let (error_type, validate_body) = match mode {
        ValidateMode::NonEmpty => (
            quote! { ::identifiers::EmptyError },
            quote! {
                if s.is_empty() {
                    ::std::result::Result::Err(::identifiers::EmptyError)
                } else {
                    ::std::result::Result::Ok(())
                }
            },
        ),
        ValidateMode::NonBlank => (
            quote! { ::identifiers::BlankError },
            quote! {
                if s.trim().is_empty() {
                    ::std::result::Result::Err(::identifiers::BlankError)
                } else {
                    ::std::result::Result::Ok(())
                }
            },
        ),
        ValidateMode::Any => (
            quote! { ::std::convert::Infallible },
            quote! { ::std::result::Result::Ok(()) },
        ),
    };

    Ok(quote! {
        #base_impls

        impl ::std::convert::TryFrom<::std::string::String> for #name {
            type Error = #error_type;

            fn try_from(s: ::std::string::String) -> ::std::result::Result<Self, Self::Error> {
                <Self as ::identifiers::StringIdentifier>::validate(&s)?;
                ::std::result::Result::Ok(Self(s))
            }
        }

        impl ::identifiers::StringIdentifier for #name {
            type Error = #error_type;

            fn validate(s: &str) -> ::std::result::Result<(), Self::Error> {
                #validate_body
            }
        }
    })
}

/// Derives [`UriIdentifier`](identifiers_uri::UriIdentifier) for a newtype wrapper around
/// [`fluent_uri::Uri<String>`].
///
/// Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `TryFrom<Uri<String>>`,
/// and `UriIdentifier`.
///
/// By default, generates a no-op `validate` (accepts all URIs); `#[validate(all)]` is an
/// explicit alias for the same. Add `#[validate(custom)]` to suppress the trait impl and
/// supply your own.
#[proc_macro_derive(UriIdentifier, attributes(validate))]
pub fn derive_uri_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uri_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uri_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let custom = parse_custom_validate_attr(input)?;

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

        impl ::std::convert::TryFrom<::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>> for #name {
            type Error = <Self as ::identifiers_uri::UriIdentifier>::Error;

            fn try_from(
                uri: ::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>,
            ) -> ::std::result::Result<Self, Self::Error> {
                <Self as ::identifiers_uri::UriIdentifier>::validate(&uri)?;
                ::std::result::Result::Ok(Self(uri))
            }
        }
    };

    if custom {
        return Ok(base_impls);
    }

    Ok(quote! {
        #base_impls

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
/// Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`,
/// `Ord`, `TryFrom<u64>`, and `IntegerIdentifier`.
///
/// By default, generates a no-op `validate` (accepts all values) and `zero()`; `#[validate(all)]`
/// is an explicit alias for the same. Add `#[validate(custom)]` to suppress the trait impl and
/// supply your own.
#[proc_macro_derive(IntegerIdentifier, attributes(validate))]
pub fn derive_integer_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_integer_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_integer_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    let custom = parse_custom_validate_attr(input)?;

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

        impl ::std::convert::TryFrom<u64> for #name {
            type Error = <Self as ::identifiers::IntegerIdentifier>::Error;

            fn try_from(n: u64) -> ::std::result::Result<Self, Self::Error> {
                <Self as ::identifiers::IntegerIdentifier>::validate(n)?;
                ::std::result::Result::Ok(Self(n))
            }
        }
    };

    if custom {
        return Ok(base_impls);
    }

    Ok(quote! {
        #base_impls

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
