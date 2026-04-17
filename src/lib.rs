use std::fmt::Debug;
use std::hash::Hash;

pub use identifiers_derive::{IntegerIdentifier, StringIdentifier};

/// Error returned when a [`StringIdentifier`] with `#[allowed_values(non_empty)]` is constructed
/// from an empty string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyError;

impl std::fmt::Display for EmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier string must not be empty")
    }
}

impl std::error::Error for EmptyError {}

/// Error returned when a [`StringIdentifier`] with `#[allowed_values(non_blank)]` is constructed
/// from a blank string (empty or all whitespace).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlankError;

impl std::fmt::Display for BlankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("identifier string must not be blank")
    }
}

impl std::error::Error for BlankError {}

/// Returns `Ok(())` if `s` is non-empty, or [`EmptyError`] otherwise.
///
/// Intended for use inside custom [`StringIdentifier::validate`] implementations:
///
/// ```rust
/// # use identifiers::{StringIdentifier, EmptyError, require_non_empty};
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct MyId(String);
/// # impl AsRef<str> for MyId { fn as_ref(&self) -> &str { &self.0 } }
/// # impl StringIdentifier for MyId {
/// #     type Error = EmptyError;
///     fn validate(s: &str) -> Result<(), EmptyError> {
///         require_non_empty(s)?;
///         // additional checks …
///         Ok(())
///     }
/// # }
/// ```
pub fn require_non_empty(s: &str) -> Result<(), EmptyError> {
    if s.is_empty() {
        Err(EmptyError)
    } else {
        Ok(())
    }
}

/// Returns `Ok(())` if `s` is non-blank (contains at least one non-whitespace character),
/// or [`BlankError`] otherwise.
///
/// Intended for use inside custom [`StringIdentifier::validate`] implementations:
///
/// ```rust
/// # use identifiers::{StringIdentifier, BlankError, require_non_blank};
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # struct MyId(String);
/// # impl AsRef<str> for MyId { fn as_ref(&self) -> &str { &self.0 } }
/// # impl StringIdentifier for MyId {
/// #     type Error = BlankError;
///     fn validate(s: &str) -> Result<(), BlankError> {
///         require_non_blank(s)?;
///         // additional checks …
///         Ok(())
///     }
/// # }
/// ```
pub fn require_non_blank(s: &str) -> Result<(), BlankError> {
    if s.trim().is_empty() {
        Err(BlankError)
    } else {
        Ok(())
    }
}

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
    ///
    /// Called automatically by the derived `From`/`TryFrom` conversion impls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers::{StringIdentifier, EmptyError};
    ///
    /// #[derive(StringIdentifier)]
    /// #[allowed_values(non_empty)]
    /// struct UserId(String);
    ///
    /// assert!(UserId::validate("alice").is_ok());
    /// assert_eq!(UserId::validate(""), Err(EmptyError));
    /// ```
    fn validate(s: &str) -> Result<(), Self::Error>;

    /// Returns the underlying string slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers::StringIdentifier;
    ///
    /// #[derive(StringIdentifier)]
    /// #[allowed_values(all)]
    /// struct UserId(String);
    ///
    /// let id = UserId::from("alice".to_string());
    /// assert_eq!(id.as_str(), "alice");
    /// ```
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

/// Common interface for typed `u64` wrappers used as identifiers or sequence positions.
///
/// Implies [`Ord`] and [`PartialOrd`]; implementors must derive or implement those traits.
///
/// # Examples
///
/// ```rust
/// use identifiers::IntegerIdentifier;
///
/// #[derive(IntegerIdentifier)]
/// #[allowed_values(all)]
/// struct InvoiceId(u64);
///
/// let id = InvoiceId::new(1042);
/// assert_eq!(id.as_u64(), 1042);
/// assert!(InvoiceId::zero() < id);
/// ```
pub trait IntegerIdentifier:
    Debug + Clone + Copy + PartialEq + Eq + Hash + Ord + PartialOrd
{
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `n` before it is wrapped. Return `Err` to reject it.
    ///
    /// Called automatically by the derived `From`/`TryFrom` conversion impls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers::IntegerIdentifier;
    ///
    /// #[derive(IntegerIdentifier)]
    /// #[allowed_values(all)]
    /// struct InvoiceId(u64);
    ///
    /// assert!(InvoiceId::validate(42).is_ok());
    /// ```
    fn validate(n: u64) -> Result<(), Self::Error>;

    /// Returns an instance initialised to zero (the smallest valid value).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers::IntegerIdentifier;
    ///
    /// #[derive(IntegerIdentifier)]
    /// #[allowed_values(all)]
    /// struct InvoiceId(u64);
    ///
    /// assert_eq!(InvoiceId::zero().as_u64(), 0);
    /// assert!(InvoiceId::zero() <= InvoiceId::new(1));
    /// ```
    fn zero() -> Self;

    /// Returns the underlying `u64`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers::IntegerIdentifier;
    ///
    /// #[derive(IntegerIdentifier)]
    /// #[allowed_values(all)]
    /// struct InvoiceId(u64);
    ///
    /// assert_eq!(InvoiceId::new(42).as_u64(), 42);
    /// ```
    fn as_u64(&self) -> u64;
}
