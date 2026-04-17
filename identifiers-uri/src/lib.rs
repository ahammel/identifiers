use std::fmt::Debug;
use std::hash::Hash;

use fluent_uri::Uri;

pub use identifiers_derive::UriIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use fluent_uri;
}

/// Common interface for typed URI wrappers.
pub trait UriIdentifier: Debug + Clone + PartialEq + Eq + Hash + From<Uri<String>> {
    /// Returns the underlying [`Uri`].
    fn as_uri(&self) -> &Uri<String>;
}
