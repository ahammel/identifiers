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
/// Construct instances via [`TryFrom<String>`]; the `validate` method is called automatically
/// at construction time. Use the `#[derive(StringIdentifier)]` macro with a `#[validate(...)]`
/// attribute to generate the implementation:
///
/// - `#[validate(non_empty)]` — rejects empty strings; error type is [`EmptyError`]
/// - `#[validate(non_blank)]` — rejects blank strings (empty or all whitespace); error type
///   is [`BlankError`]
/// - `#[validate(any)]` — accepts all strings; error type is [`Infallible`](std::convert::Infallible)
///
/// For custom validation, use `#[validate(custom)]` or omit the attribute entirely, then
/// implement `validate` and `as_str` manually; the derive will still generate the boilerplate
/// (`Debug`, `Clone`, etc.) and the `TryFrom<String>` impl.
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
