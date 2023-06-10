use crate::capacity::Capacity;

mod alloc;
#[cfg(feature = "alloc")]
pub use self::alloc::AllocStorage;

mod array;
pub use array::ArrayStorage;

mod impls;

/// A generic backing storage for ring buffers.
pub unsafe trait Storage {
    /// The type of the items held by this storage.
    type Item;

    /// The type of this storage's capacity.
    type Capacity: Capacity;

    /// Get the capacity for this storage.
    fn capacity(&self) -> Self::Capacity;

    /// Get a `const` pointer to the stored elements safely.
    fn get_ptr(&self) -> *const [Self::Item];

    /// Get a `mut` pointer to the stored elements safely.
    fn get_ptr_mut(&mut self) -> *mut [Self::Item];
}
