use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

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

/// Derives [`UuidIdentifier`](identifiers_uuid::UuidIdentifier) for a newtype wrapper around
/// [`uuid::Uuid`].
///
/// Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `From<uuid::Uuid>`,
/// and `UuidIdentifier`.
#[proc_macro_derive(UuidIdentifier)]
pub fn derive_uuid_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uuid_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uuid_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    Ok(quote! {
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

        impl ::std::convert::From<::identifiers_uuid::__private::uuid::Uuid> for #name {
            fn from(id: ::identifiers_uuid::__private::uuid::Uuid) -> Self { Self(id) }
        }

        impl ::identifiers_uuid::UuidIdentifier for #name {
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
/// Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `From<String>`,
/// and `StringIdentifier`.
#[proc_macro_derive(StringIdentifier)]
pub fn derive_string_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_string_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_string_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    Ok(quote! {
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

        impl ::std::convert::From<::std::string::String> for #name {
            fn from(id: ::std::string::String) -> Self { Self(id) }
        }

        impl ::identifiers::StringIdentifier for #name {
            fn as_str(&self) -> &str { &self.0 }
        }
    })
}

/// Derives [`UriIdentifier`](identifiers_uri::UriIdentifier) for a newtype wrapper around
/// [`fluent_uri::Uri<String>`].
///
/// Implements `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `From<Uri<String>>`,
/// and `UriIdentifier`.
#[proc_macro_derive(UriIdentifier)]
pub fn derive_uri_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_uri_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_uri_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    Ok(quote! {
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

        impl ::std::convert::From<::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>> for #name {
            fn from(uri: ::identifiers_uri::__private::fluent_uri::Uri<::std::string::String>) -> Self {
                Self(uri)
            }
        }

        impl ::identifiers_uri::UriIdentifier for #name {
            fn as_uri(&self) -> &::identifiers_uri::__private::fluent_uri::Uri<::std::string::String> {
                &self.0
            }
        }
    })
}

/// Derives [`IntegerIdentifier`](identifiers::IntegerIdentifier) for a newtype wrapper around
/// `u64`.
///
/// Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`,
/// `Ord`, `From<u64>`, and `IntegerIdentifier`.
#[proc_macro_derive(IntegerIdentifier)]
pub fn derive_integer_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_integer_identifier_inner(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn derive_integer_identifier_inner(input: &DeriveInput) -> Result<TokenStream2, Error> {
    require_newtype(input)?;
    let name = &input.ident;
    Ok(quote! {
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

        impl ::std::convert::From<u64> for #name {
            fn from(id: u64) -> Self { Self(id) }
        }

        impl ::identifiers::IntegerIdentifier for #name {
            fn zero() -> Self { Self(0) }

            fn as_u64(&self) -> u64 { self.0 }
        }
    })
}
