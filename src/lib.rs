use std::fmt::Debug;
use std::hash::Hash;

pub use identifiers_derive::{IntegerIdentifier, StringIdentifier};

/// Error returned when a [`StringIdentifier`] with `#[validate(non_empty)]` is constructed
/// from an empty string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyError;

impl std::fmt::Display for EmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier string must not be empty")
    }
}

impl std::error::Error for EmptyError {}

/// Error returned when a [`StringIdentifier`] with `#[validate(non_blank)]` is constructed
/// from a blank string (empty or all whitespace).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankError;

impl std::fmt::Display for BlankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier string must not be blank")
    }
}

impl std::error::Error for BlankError {}

/// Common interface for typed string wrappers used as identifiers.
///
/// Use `#[derive(StringIdentifier)]` to generate the implementation. The derive always emits
/// the boilerplate (`Debug`, `Clone`, etc.) and the `StringIdentifier` impl. Add an
/// `#[allowed_values(...)]` attribute to also derive a conversion impl:
///
/// - `#[allowed_values(all)]` — derives `From<String>`
/// - `#[allowed_values(non_empty)]` — derives `TryFrom<String>`, rejects empty strings
/// - `#[allowed_values(non_blank)]` — derives `TryFrom<String>`, rejects blank strings
pub trait StringIdentifier: Debug + Clone + PartialEq + Eq + Hash + AsRef<str> {
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `s` before it is wrapped. Return `Err` to reject the string.
    fn validate(s: &str) -> Result<(), Self::Error>;

    /// Returns the underlying string slice.
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

/// Common interface for typed `u64` wrappers used as identifiers or sequence positions.
///
/// Implies [`Ord`] and [`PartialOrd`]; implementors must derive or implement those traits.
pub trait IntegerIdentifier:
    Debug + Clone + Copy + PartialEq + Eq + Hash + Ord + PartialOrd
{
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `n` before it is wrapped. Return `Err` to reject it.
    fn validate(n: u64) -> Result<(), Self::Error>;

    /// Returns an instance initialised to zero (the smallest valid value).
    fn zero() -> Self;

    /// Returns the underlying `u64`.
    fn as_u64(&self) -> u64;
}
