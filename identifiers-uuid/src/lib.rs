use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use identifiers_derive::UuidIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use uuid;
}

/// Common interface for typed UUID wrappers.
pub trait UuidIdentifier: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// The error type returned when validation fails.
    type Error: std::error::Error;

    /// Validates `uuid` before it is wrapped. Return `Err` to reject it.
    fn validate(uuid: &Uuid) -> Result<(), Self::Error>;

    /// Generates a new random identifier.
    fn new() -> Self;

    /// Returns the underlying [`Uuid`].
    fn as_uuid(&self) -> Uuid;
}
