use super::{FullStorage, PartialStorage, Storage};

unsafe impl<T: Storage> Storage for &mut T {
    type Item = T::Item;
    type Capacity = T::Capacity;

    fn capacity(&self) -> Self::Capacity {
        T::capacity(*self)
    }
}

unsafe impl<T: PartialStorage> PartialStorage for &mut T {
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
