use core::num::NonZeroUsize;

use crate::capacity::NonZeroCapacity;
use crate::storage::{ArrayStorage, Storage};

mod tests;

/// A [`SubtractingRingBuffer`] backed by [`ArrayStorage`].
pub type SubtractingArrayRingBuffer<T, const N: usize> =
    SubtractingRingBuffer<ArrayStorage<T, NonZeroCapacity, N>>;

/// A ring buffer based on conditional subtraction.
///
/// In order to bring indices into range, this ring buffer will conditionally subtract them.  This
/// is unlike [`MaskingRingBuffer`](crate::masking::MaskingRingBuffer), which uses bitwise masking
/// to the same effect.  [`SubtractingRingBuffer`] supports capacity sizes that are not powers of
/// two.
pub struct SubtractingRingBuffer<S>
where S: ?Sized + Storage<Capacity = NonZeroCapacity> {
    /// The offset of the items in storage.
    ///
    /// The items begin at this offset (in units of elements), possibly looping around.  Its value
    /// is strictly less than the storage capacity.
    off: usize,

    /// The number of items in storage.
    ///
    /// There are exactly this number of items currently in storage.  Its value is less than or
    /// equal to the storage capacity.
    len: usize,

    /// Storage for the buffer's items.
    storage: S,
}

impl<S> SubtractingRingBuffer<S>
where S: ?Sized + Storage<Capacity = NonZeroCapacity> {
    /// Whether the ring buffer is full.
    ///
    /// The ring buffer is considered full if it has as many elements as its [`capacity()`].  At
    /// this point, [`enqueue()`]-ing new elements will cause older elements to be removed and
    /// returned.
    ///
    /// [`capacity()`]: SubtractingRingBuffer::capacity()
    /// [`enqueue()`]: SubtractingRingBuffer::enqueue()
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
    pub fn capacity(&self) -> usize {
        NonZeroUsize::from(self.storage.capacity()).get()
    }

    /// Append an element to the ring buffer.
    ///
    /// If the ring buffer is full (see [`is_full()`]), the oldest element in the ring buffer will
    /// be removed and returned in [`Some`]; if the ring buffer was not full, [`None`] is returned.
    ///
    /// [`is_full()`]: SubtractingRingBuffer::is_full()
    pub fn enqueue(&mut self, item: S::Item) -> Option<S::Item> {
        let (off, len, cap) = (self.off, self.len, self.capacity());

        // The position the element has to be written to.
        let pos = if len == cap {
            // pos = (off + len) % cap = (off + cap) % cap = off
            // thus pos < cap
            off
        } else {
            if off + len >= cap {
                // pos = (cap <= off + len < 2 * cap) - cap
                // thus pos < cap
                off + len - cap
            } else {
                // pos = (off + len < cap) % cap = off + len
                // thus pos < cap
                off + len
            }
        };

        // A pointer to the slot for the new element.
        let ptr = unsafe {
            // SAFETY: pos < cap, thus it is a valid index into storage.
            self.storage.get_ptr_mut().cast::<S::Item>().add(pos)
        };

        if len == cap {
            self.off = if off + 1 == cap { off + 1 - cap } else { off + 1 };
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
    /// [`enqueue()`]: SubtractingRingBuffer::enqueue()
    /// [`dequeue()`]: SubtractingRingBuffer::dequeue()
    pub fn dequeue(&mut self) -> Option<S::Item> {
        let (off, len, cap) = (self.off, self.len, self.capacity());

        if len == 0 { return None; }

        // A pointer to the slot for the old element.
        let ptr = unsafe {
            // SAFETY: off < cap, thus it is a valid index into storage.
            self.storage.get_ptr_mut().cast::<S::Item>().add(off)
        };

        self.off = if off + 1 == cap { off + 1 - cap } else { off + 1 };
        self.len -= 1;
        Some(unsafe { ptr.read() })
    }
}

impl<S> SubtractingRingBuffer<S>
where S: Storage<Capacity = NonZeroCapacity> {
    /// Construct a new [`SubtractingRingBuffer`] with the given storage.
    ///
    /// The resulting buffer is empty - elements can be filled in afterwards.  Any data in the
    /// storage will be overwritten.
    pub fn with_storage(storage: S) -> Self {
        Self {
            off: 0,
            len: 0,
            storage,
        }
    }
}

impl<S> Default for SubtractingRingBuffer<S>
where S: Default + Storage<Capacity = NonZeroCapacity> {
    fn default() -> Self {
        Self {
            off: 0,
            len: 0,
            storage: S::default(),
        }
    }
}
