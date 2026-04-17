use std::fmt::Debug;
use std::hash::Hash;

use uuid::Uuid;

pub use identifiers_derive::UuidIdentifier;

#[doc(hidden)]
pub mod __private {
    pub use uuid;
}

/// Common interface for typed UUID wrappers.
pub trait UuidIdentifier: Debug + Clone + Copy + PartialEq + Eq + Hash + From<Uuid> {
    /// Generates a new random identifier.
    fn new() -> Self;

    /// Returns the underlying [`Uuid`].
    fn as_uuid(&self) -> Uuid;
}
