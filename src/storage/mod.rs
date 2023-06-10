use crate::capacity::Capacity;

mod impls;

/// A generic backing storage for ring buffers.
pub unsafe trait Storage {
    /// The type of the items held by this storage.
    type Item;

    /// The type of this storage's capacity.
    type Capacity: Capacity;

    /// Get the capacity for this storage.
    unsafe fn capacity(this: *const Self) -> Self::Capacity;
}

/// Ring buffer storage that permits uninitialized elements.
pub unsafe trait PartialStorage: Storage {
    /// Get a `const` pointer to the stored elements.
    unsafe fn raw_ptr(this: *const Self) -> *const [Self::Item];

    /// Get a `mut` pointer to the stored elements.
    unsafe fn raw_ptr_mut(this: *mut Self) -> *mut [Self::Item];
}

/// Indirect ring buffer storage that permits uninitialized elements.
pub unsafe trait IndirectPartialStorage: PartialStorage {
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
