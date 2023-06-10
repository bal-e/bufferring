use core::marker::PhantomData;
use core::mem::MaybeUninit;

use super::Storage;
use crate::capacity::Capacity;

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

    fn capacity(&self) -> Self::Capacity {
        C::from_ct::<N>()
    }

    fn get_ptr(&self) -> *const [Self::Item] {
        self.inner.as_ptr()
    }

    fn get_ptr_mut(&mut self) -> *mut [Self::Item] {
        self.inner.as_mut_ptr()
    }
}
