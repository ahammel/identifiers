use std::fmt::Debug;
use std::hash::Hash;

use fluent_uri::Uri;

pub use identifiers_derive::UriIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use fluent_uri;
}

/// Common interface for typed URI wrappers.
///
/// # Examples
///
/// ```rust
/// use identifiers_uri::UriIdentifier;
/// use fluent_uri::Uri;
///
/// #[derive(UriIdentifier)]
/// #[allowed_values(all)]
/// struct LinkId(Uri<String>);
///
/// let uri = Uri::<String>::parse("https://example.com".to_string()).unwrap();
/// let id = LinkId::from(uri);
/// assert_eq!(id.as_uri().as_str(), "https://example.com");
/// ```
pub trait UriIdentifier: Debug + Clone + PartialEq + Eq + Hash {
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `uri` before it is wrapped. Return `Err` to reject it.
    ///
    /// Called automatically by the derived `From` conversion impl.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers_uri::UriIdentifier;
    /// use fluent_uri::Uri;
    ///
    /// #[derive(UriIdentifier)]
    /// #[allowed_values(all)]
    /// struct LinkId(Uri<String>);
    ///
    /// let uri = Uri::<String>::parse("https://example.com".to_string()).unwrap();
    /// assert!(LinkId::validate(&uri).is_ok());
    /// ```
    fn validate(uri: &Uri<String>) -> Result<(), Self::Error>;

    /// Returns the underlying [`Uri`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers_uri::UriIdentifier;
    /// use fluent_uri::Uri;
    ///
    /// #[derive(UriIdentifier)]
    /// #[allowed_values(all)]
    /// struct LinkId(Uri<String>);
    ///
    /// let uri = Uri::<String>::parse("https://example.com".to_string()).unwrap();
    /// let id = LinkId::from(uri);
    /// assert_eq!(id.as_uri().as_str(), "https://example.com");
    /// ```
    fn as_uri(&self) -> &Uri<String>;
}
