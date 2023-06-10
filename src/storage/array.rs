use core::marker::PhantomData;
use core::mem::MaybeUninit;

use crate::capacity::Capacity;
use super::{Storage, PartialStorage, IndirectPartialStorage};

/// Ring buffer storage backed by a fixed-size array.
pub struct ArrayStorage<T, C: Capacity, const N: usize> {
    inner: MaybeUninit<[T; N]>,
    _capacity: PhantomData<C>,
}

impl<T, C: Capacity, const N: usize> Default for ArrayStorage<T, C, N> {
    fn default() -> Self {
        // Ensure that `N` is a valid capacity for this storage.
        let _ = C::from_ct::<N>();

        Self {
            inner: MaybeUninit::uninit(),
            _capacity: PhantomData,
        }
    }
}

unsafe impl<T, C: Capacity, const N: usize> Storage for ArrayStorage<T, C, N> {
    type Item = T;
    type Capacity = C;

    unsafe fn capacity(_: *const Self) -> Self::Capacity {
        C::from_ct::<N>()
    }
}

unsafe impl<T, C: Capacity, const N: usize> PartialStorage for ArrayStorage<T, C, N> {
    unsafe fn raw_ptr(this: *const Self) -> *const [Self::Item] {
        // SAFETY: Only the elements of 'this' can be uninitialized, but that's
        // all wrapped in 'MaybeUninit', so we can cast to a regular reference.
        (&*this).inner.as_ptr()
    }

    unsafe fn raw_ptr_mut(this: *mut Self) -> *mut [Self::Item] {
        // SAFETY: Only the elements of 'this' can be uninitialized, but that's
        // all wrapped in 'MaybeUninit', so we can cast to a regular reference.
        (&mut *this).inner.as_mut_ptr()
    }
}

unsafe impl<T, C: Capacity, const N: usize> IndirectPartialStorage for ArrayStorage<T, C, N> {
    fn get_ptr(&self) -> *const [Self::Item] {
        self.inner.as_ptr()
    }

    fn get_ptr_mut(&mut self) -> *mut [Self::Item] {
        self.inner.as_mut_ptr()
    }
}
