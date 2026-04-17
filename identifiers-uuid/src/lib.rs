use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use identifiers_derive::UuidIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use uuid;
}

/// Common interface for typed UUID wrappers.
///
/// # Examples
///
/// ```rust
/// use identifiers_uuid::UuidIdentifier;
/// use uuid::Uuid;
///
/// #[derive(UuidIdentifier)]
/// #[allowed_values(all)]
/// struct EventId(Uuid);
///
/// let id = EventId::new();
/// assert_ne!(id.as_uuid(), Uuid::nil());
/// ```
pub trait UuidIdentifier: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `uuid` before it is wrapped. Return `Err` to reject it.
    ///
    /// Called automatically by the derived `From` conversion impl.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers_uuid::UuidIdentifier;
    /// use uuid::Uuid;
    ///
    /// #[derive(UuidIdentifier)]
    /// #[allowed_values(all)]
    /// struct EventId(Uuid);
    ///
    /// assert!(EventId::validate(&Uuid::new_v4()).is_ok());
    /// ```
    fn validate(uuid: &Uuid) -> Result<(), Self::Error>;

    /// Generates a new random (v4) identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers_uuid::UuidIdentifier;
    /// use uuid::Uuid;
    ///
    /// #[derive(UuidIdentifier)]
    /// #[allowed_values(all)]
    /// struct EventId(Uuid);
    ///
    /// // Each call produces a distinct value.
    /// assert_ne!(EventId::new(), EventId::new());
    /// ```
    fn new() -> Self;

    /// Returns the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use identifiers_uuid::UuidIdentifier;
    /// use uuid::Uuid;
    ///
    /// #[derive(UuidIdentifier)]
    /// #[allowed_values(all)]
    /// struct EventId(Uuid);
    ///
    /// let uuid = Uuid::new_v4();
    /// assert_eq!(EventId::from(uuid).as_uuid(), uuid);
    /// ```
    fn as_uuid(&self) -> Uuid;
}
