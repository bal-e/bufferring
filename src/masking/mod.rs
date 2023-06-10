mod tests;

pub struct MaskingRingBuffer<S: PartialStorage<Capacity = MaskingCapacity>> {
    /// The start of the buffer in the storage (`0..CAPACITY`)
    index: usize,
    /// The number of items in the buffer (`0..=CAPACITY`)
    len: usize,
    /// The underlying storage
    storage: MaybeUninit<S>,
}

impl<S: PartialStorage<Capacity = MaskingCapacity>> MaskingRingBuffer<S> {
    pub fn new(storage: S) -> Self {
        MaskingRingBuffer {
            storage: MaybeUninit::new(storage),
            index: 0,
            len: 0,
        }
    }

    /// Returns whether the ringbuffer is full
    ///
    /// A ringbuffer is full when its length equals its capacity. If an item is enqueued
    /// while the ringbuffer is full [MaskingRingBuffer::enqueue] will dequeue an item to
    /// make room for the new item. The dequeued item will be returned.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity().get()
    }

    /// Returns true when the ringbuffer is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The capacity of the underlying storage
    ///
    /// This is the maximum number of items that the ringbuffer can hold.
    pub fn capacity(&self) -> NonZeroUsize {
        unsafe { Storage::capacity(self.storage.as_ptr()) }.into()
    }

    /// Add an element to the end of the ringbuffer
    ///
    /// If the ringbuffer is full, the first-in element will be removed from the buffer and returned.
    pub fn enqueue(&mut self, item: S::Item) -> Option<S::Item> {
        // We need to dequeue if the buffer is full, in which case we return dequeued item
        let dequeued_item = self.is_full().then(|| self.dequeue()).flatten();

        // Find the offset to put the new item on
        let mask = unsafe { Storage::capacity(self.storage.as_ptr()) }.mask();
        let offset = mask & (self.index + self.len);

        // Write the item into the storage at offset
        // TODO: This could be a provided method on PartialStorage
        let buffer = unsafe { PartialStorage::raw_ptr_mut(self.storage.as_mut_ptr()) };
        let ptr = unsafe { (buffer as *mut S::Item).add(offset) };
        unsafe { *ptr = item };

        self.len += 1;

        dequeued_item
    }

    /// Remove an element from the start of the ringbuffer
    pub fn dequeue(&mut self) -> Option<S::Item> {
        if self.is_empty() {
            return None;
        }

        // Get the item from the buffer
        let offset = self.index.0;
        let buffer = unsafe { PartialStorage::raw_ptr_mut(self.storage.as_mut_ptr()) };
        let ptr = unsafe { (buffer as *mut MaybeUninit<S::Item>).add(offset) };
        let item = unsafe { core::ptr::replace(ptr, MaybeUninit::uninit()).assume_init() };

        let mask = unsafe { Storage::capacity(self.storage.as_ptr()) }.mask();
        self.index = mask & (self.index + 1);
        self.len -= 1;

        Some(item)
    }
}
