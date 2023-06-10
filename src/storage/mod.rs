use crate::capacity::Capacity;

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
}

/// Ring buffer storage that permits uninitialized elements.
pub unsafe trait PartialStorage: Storage {
    /// Get a `const` pointer to the stored elements safely.
    fn get_ptr(&self) -> *const [Self::Item];

    /// Get a `mut` pointer to the stored elements safely.
    fn get_ptr_mut(&mut self) -> *mut [Self::Item];
}

/// Ring buffer storage that can only hold initialized elements.
pub unsafe trait FullStorage: Storage {
    /// Get a shared reference to the stored elements.
    fn get(&self) -> &[Self::Item];

    /// Get a unique reference to the stored elements.
    fn get_mut(&mut self) -> &mut [Self::Item];
}
