#![cfg(feature = "alloc")]

use core::num::NonZeroUsize;
use core::ptr::{self, NonNull};

use ::alloc::alloc::{self, Layout};

use super::{PartialStorage, Storage};
use crate::capacity::Capacity;

/// Ring buffer storage backed by dynamic allocation.
pub struct AllocStorage<T, C: Capacity> {
    /// A pointer to the allocated data.
    pointer: NonNull<T>,
    /// The storage capacity.
    capacity: C,
}

impl<T, C: Capacity> AllocStorage<T, C> {
    /// Allocate storage for a ring buffer.
    pub fn new(capacity: C) -> Self {
        let raw_capacity = NonZeroUsize::get(capacity.into());
        let layout =
            Layout::array::<T>(raw_capacity).expect("Layout calculation failed due to overflow");
        if layout.size() == 0 {
            return Self {
                pointer: NonNull::dangling(),
                capacity,
            };
        }

        // SAFETY: We confirmed above that 'layout' has a non-zero size.
        let raw_pointer = unsafe { alloc::alloc(layout) } as *mut T;
        let Some(pointer) = NonNull::new(raw_pointer) else {
            alloc::handle_alloc_error(layout)
        };
        Self { pointer, capacity }
    }
}

unsafe impl<T, C: Capacity> Storage for AllocStorage<T, C> {
    type Item = T;
    type Capacity = C;

    fn capacity(&self) -> Self::Capacity {
        self.capacity
    }
}

unsafe impl<T, C: Capacity> PartialStorage for AllocStorage<T, C> {
    fn get_ptr(&self) -> *const [Self::Item] {
        let raw_capacity = NonZeroUsize::get(self.capacity.into());
        ptr::slice_from_raw_parts(self.pointer.as_ptr(), raw_capacity)
    }

    fn get_ptr_mut(&mut self) -> *mut [Self::Item] {
        let raw_capacity = NonZeroUsize::get(self.capacity.into());
        ptr::slice_from_raw_parts_mut(self.pointer.as_ptr(), raw_capacity)
    }
}
