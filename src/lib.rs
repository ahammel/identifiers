use std::fmt::Debug;
use std::hash::Hash;

pub use identifiers_derive::{IntegerIdentifier, StringIdentifier};

/// Common interface for typed string wrappers used as identifiers.
pub trait StringIdentifier: Debug + Clone + PartialEq + Eq + Hash + From<String> {
    /// Returns the underlying string slice.
    fn as_str(&self) -> &str;
}

/// Common interface for typed `u64` wrappers used as identifiers or sequence positions.
///
/// Implies [`Ord`] and [`PartialOrd`]; implementors must derive or implement those traits.
pub trait IntegerIdentifier:
    Debug + Clone + Copy + PartialEq + Eq + Hash + Ord + PartialOrd + From<u64>
{
    /// Returns an instance initialised to zero (the smallest valid value).
    fn zero() -> Self;

    /// Returns the underlying `u64`.
    fn as_u64(&self) -> u64;
}
