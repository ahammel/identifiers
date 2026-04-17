use std::fmt::Debug;
use std::hash::Hash;

use fluent_uri::Uri;

pub use identifiers_derive::UriIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use fluent_uri;
}

/// Common interface for typed URI wrappers.
pub trait UriIdentifier: Debug + Clone + PartialEq + Eq + Hash {
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `uri` before it is wrapped. Return `Err` to reject it.
    fn validate(uri: &Uri<String>) -> Result<(), Self::Error>;

    /// Returns the underlying [`Uri`].
    fn as_uri(&self) -> &Uri<String>;
}
