use core::mem::MaybeUninit;

use super::{Storage, PartialStorage, IndirectPartialStorage, FullStorage};

unsafe impl<T: Storage> Storage for &mut T {
    type Item = T::Item;
    type Capacity = T::Capacity;

    unsafe fn capacity(this: *const Self) -> Self::Capacity {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized; this does not include the mutable reference itself, so
        // we can read from it.
        T::capacity(*(this as *const *const T))
    }
}

unsafe impl<T: IndirectPartialStorage> PartialStorage for &mut T {
    unsafe fn raw_ptr(this: *const Self) -> *const [Self::Item] {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized; this does not include the mutable reference itself, so
        // we can read from it.
        T::raw_ptr(*(this as *const *const T))
    }

    unsafe fn raw_ptr_mut(this: *mut Self) -> *mut [Self::Item] {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized; this does not include the mutable reference itself, so
        // we can read from it.
        T::raw_ptr_mut(*(this as *mut *mut T))
    }
}

unsafe impl<T: IndirectPartialStorage> IndirectPartialStorage for &mut T {
    fn get_ptr(&self) -> *const [Self::Item] {
        T::get_ptr(*self)
    }

    fn get_ptr_mut(&mut self) -> *mut [Self::Item] {
        T::get_ptr_mut(*self)
    }
}

unsafe impl<T: FullStorage> FullStorage for &mut T {
    fn get(&self) -> &[Self::Item] {
        T::get(*self)
    }

    fn get_mut(&mut self) -> &mut [Self::Item] {
        T::get_mut(*self)
    }
}

unsafe impl<T: Storage> Storage for MaybeUninit<T> {
    type Item = T::Item;
    type Capacity = T::Capacity;

    unsafe fn capacity(this: *const Self) -> Self::Capacity {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized, so we can extract the capacity safely.
        T::capacity(this as *const T)
    }
}

unsafe impl<T: PartialStorage> PartialStorage for MaybeUninit<T> {
    unsafe fn raw_ptr(this: *const Self) -> *const [Self::Item] {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized, so we can extract the actual raw pointer safely.
        T::raw_ptr(this as *const T)
    }

    unsafe fn raw_ptr_mut(this: *mut Self) -> *mut [Self::Item] {
        // SAFETY: The caller guarantees that only the exposed elements can be
        // uninitialized, so we can extract the actual raw pointer safely.
        T::raw_ptr_mut(this as *mut T)
    }
}
