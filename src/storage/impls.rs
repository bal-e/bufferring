use super::Storage;

unsafe impl<T: Storage> Storage for &mut T {
    type Item = T::Item;
    type Capacity = T::Capacity;

    fn capacity(&self) -> Self::Capacity {
        T::capacity(*self)
    }

    fn get_ptr(&self) -> *const [Self::Item] {
        T::get_ptr(*self)
    }

    fn get_ptr_mut(&mut self) -> *mut [Self::Item] {
        T::get_ptr_mut(*self)
    }
}
