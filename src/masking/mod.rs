use core::num::NonZeroUsize;

use crate::capacity::MaskingCapacity;
use crate::storage::{ArrayStorage, Storage};

mod tests;

pub type MaskingArrayRingBuffer<T, const N: usize> =
    MaskingRingBuffer<ArrayStorage<T, MaskingCapacity, N>>;

pub struct MaskingRingBuffer<S: Storage<Capacity = MaskingCapacity>> {
    /// The start of the buffer in the storage (`0..CAPACITY`)
    index: usize,
    /// The number of items in the buffer (`0..=CAPACITY`)
    len: usize,
    /// The underlying storage
    storage: S,
}

impl<S: Storage<Capacity = MaskingCapacity>> MaskingRingBuffer<S> {
    pub fn from_empty(storage: S) -> Self {
        MaskingRingBuffer {
            index: 0,
            len: 0,
            storage,
        }
    }

    /// Returns whether the ringbuffer is full
    ///
    /// A ringbuffer is full when its length equals its capacity. If an item is enqueued while the
    /// ringbuffer is full [MaskingRingBuffer::enqueue] will dequeue an item to make room for the
    /// new item. The dequeued item will be returned.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Returns true when the ringbuffer is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The capacity of the underlying storage
    ///
    /// This is the maximum number of items that the ringbuffer can hold.
    pub fn capacity(&self) -> usize {
        NonZeroUsize::from(self.storage.capacity()).get()
    }

    /// Add an element to the end of the ringbuffer
    ///
    /// If the ringbuffer is full, the first-in element will be removed from the buffer and
    /// returned.
    pub fn enqueue(&mut self, item: S::Item) -> Option<S::Item> {
        let mask = self.storage.capacity().mask();
        let offset = mask & (self.index + self.len);
        let buffer = self.storage.get_ptr_mut();

        // SAFETY: Because the offset is masked, it is within the capacity and hence within the
        // storage. We also know that buffer is a valid pointer, because it comes from a valid
        // PartialStorage.
        let ptr = unsafe { buffer.cast::<S::Item>().add(offset) };

        if self.is_full() {
            self.index = mask & (self.index + 1);
            Some(unsafe { ptr.replace(item) })
        } else {
            unsafe { *ptr = item };
            self.len += 1;
            None
        }
    }

    /// Remove an element from the start of the ringbuffer
    pub fn dequeue(&mut self) -> Option<S::Item> {
        if self.is_empty() {
            return None;
        }

        // Get the item from the buffer
        let buffer = self.storage.get_ptr_mut();
        let item = unsafe { buffer.cast::<S::Item>().add(self.index).read() };

        let mask = self.storage.capacity().mask();
        self.index = mask & (self.index + 1);
        self.len -= 1;

        Some(item)
    }
}

impl<S: Storage<Capacity = MaskingCapacity> + Default> Default for MaskingRingBuffer<S> {
    fn default() -> Self {
        MaskingRingBuffer {
            index: 0,
            len: 0,
            storage: S::default(),
        }
    }
}
