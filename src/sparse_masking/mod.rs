use core::num::NonZeroUsize;

use crate::capacity::{MaskingCapacity, NonZeroCapacity};
use crate::storage::{ArrayStorage, Storage};

mod tests;

/// A [`SparseMaskingRingBuffer`] backed by [`ArrayStorage`].
pub type SparseMaskingArrayRingBuffer<T, const N: usize> =
    SparseMaskingRingBuffer<ArrayStorage<T, MaskingCapacity, N>>;

/// A sparse ring buffer based on masking.
///
/// Masking is an efficient strategy for wrapping indices for ring buffers.  However, masking
/// requires the use of a power-of-two capacity.  [`SparseMaskingRingBuffer`] allows the use of
/// non-power-of-two ring buffer sizes with masking by always leaving empty space in a power-of-two
/// masking buffer.
pub struct SparseMaskingRingBuffer<S>
where S: ?Sized + Storage<Capacity = MaskingCapacity> {
    /// The offset of the items in storage.
    ///
    /// The items begin at this offset (in units of elements), possibly looping around.  Its value
    /// is strictly less than the storage capacity.
    off: usize,

    /// The number of items in storage.
    ///
    /// There are exactly this number of items currently in storage.  Its value is less than or
    /// equal to the artificial capacity.
    len: usize,

    /// The artificial capacity of the storage.
    ///
    /// This is a limit imposed by the ring buffer in order to use non-power-of-two buffer sizes.
    /// Its value is less than or equal to the storage capacity.
    cap: NonZeroCapacity,

    /// Storage for the buffer's items.
    storage: S,
}

impl<S> SparseMaskingRingBuffer<S>
where S: ?Sized + Storage<Capacity = MaskingCapacity> {
    /// Whether the ring buffer is full.
    ///
    /// The ring buffer is considered full if it has as many elements as its [`capacity()`].  At
    /// this point, [`enqueue()`]-ing new elements will cause older elements to be removed and
    /// returned.
    ///
    /// [`capacity()`]: SparseMaskingRingBuffer::capacity()
    /// [`enqueue()`]: SparseMaskingRingBuffer::enqueue()
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Whether the ring buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The ring buffer's capacity.
    ///
    /// This is the maximum number of elements the ring buffer can ever hold.  This value is
    /// constant - it will never change for any ring buffer instance.
    ///
    /// Note that the ring buffer is backed by storage which may have a larger capacity.
    pub fn capacity(&self) -> usize {
        NonZeroUsize::from(self.cap).get()
    }

    /// Append an element to the ring buffer.
    ///
    /// If the ring buffer is full (see [`is_full()`]), the oldest element in the ring buffer will
    /// be removed and returned in [`Some`]; if the ring buffer was not full, [`None`] is returned.
    ///
    /// [`is_full()`]: SparseMaskingRingBuffer::is_full()
    pub fn enqueue(&mut self, item: S::Item) -> Option<S::Item> {
        let (off, len) = (self.off, self.len);
        let mask = self.storage.capacity().mask();

        // The position the element has to be written to.
        let pos = (off + len) & mask;

        // A pointer to the slot for the new element.
        let ptr = unsafe {
            // SAFETY: pos < cap, thus it is a valid index into storage.
            self.storage.get_ptr_mut().cast::<S::Item>().add(pos)
        };

        if self.is_full() {
            self.off = (self.off + 1) & mask;
            Some(unsafe { ptr.replace(item) })
        } else {
            unsafe { ptr.write(item) };
            self.len += 1;
            None
        }
    }

    /// Remove the oldest item from the ring buffer.
    ///
    /// If the ring buffer is not empty, the oldest element is removed and returned in [`Some`]; if
    /// the ring buffer was not full, [`None`] is returned.  Upon return, the ring buffer will not
    /// be full.
    ///
    /// Note that it is unnecessary to [`dequeue()`] before calling [`enqueue()`]; [`enqueue()`]
    /// will remove and return the oldest element if it necessary.
    ///
    /// [`enqueue()`]: SparseMaskingRingBuffer::enqueue()
    /// [`dequeue()`]: SparseMaskingRingBuffer::dequeue()
    pub fn dequeue(&mut self) -> Option<S::Item> {
        let (off, len) = (self.off, self.len);
        let mask = self.storage.capacity().mask();

        if len == 0 { return None; }

        // A pointer to the slot for the old element.
        let ptr = unsafe {
            // SAFETY: off < cap, thus it is a valid index into storage.
            self.storage.get_ptr_mut().cast::<S::Item>().add(off)
        };

        self.off = (off + 1) & mask;
        self.len -= 1;
        Some(unsafe { ptr.read() })
    }
}

impl<S> SparseMaskingRingBuffer<S>
where S: Storage<Capacity = MaskingCapacity> {
    /// Construct a new [`SparseMaskingRingBuffer`] with the given storage and capacity.
    ///
    /// The specified capacity must be less than or equal to the capacity of the storage.  The
    /// resulting buffer is empty - elements can be filled in afterwards.  Any data in the storage
    /// will be overwritten.
    ///
    /// # Panics
    ///
    /// This function will panic if the given capacity is greater than the storage capacity.
    pub fn with_storage(
        capacity: NonZeroCapacity,
        storage: S,
    ) -> Self {
        let artificial_capacity = NonZeroUsize::from(capacity);
        let storage_capacity = NonZeroUsize::from(storage.capacity());
        assert!(artificial_capacity <= storage_capacity);
        Self {
            off: 0,
            len: 0,
            cap: capacity,
            storage,
        }
    }
}
