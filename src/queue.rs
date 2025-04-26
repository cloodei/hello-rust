//! Double-ended queue (deque) implementation in Rust
//! using a vector as the underlying storage.

#[derive(Debug)]
pub struct Queue<T> {
    pub items: Vec<T>,
    pub front: usize,
    pub back: usize,
    pub size: usize
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

impl<T: Clone> Queue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Queue {
            items: Vec::with_capacity(4),
            front: 0,
            back: 0,
            size: 0,
        }
    }

    /// Creates a new empty queue with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            items: Vec::with_capacity(capacity),
            front: 0,
            back: 0,
            size: 0,
        }
    }

    /// Creates a new queue from an existing vector.
    pub fn from_vec(vec: Vec<T>) -> Self {
        Queue::from_slice(&vec)
    }

    /// Creates a new queue from an existing slice.
    pub fn from_slice(slice: &[T]) -> Self {
        let mut queue = Queue::with_capacity(slice.len());
        unsafe {
            queue.items.set_len(slice.len());
        }
        queue.items.clone_from_slice(slice);
        queue.size = slice.len();
        queue.back = slice.len();
        queue.front = 0;

        queue
    }

    /// Creates a new queue from an existing iterator.
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }

    /// Adds an element to the back of the queue.
    pub fn push_back(&mut self, value: T) {
        if self.size == self.capacity() {
            self.resize();
        }
        
        self.items[self.back] = value.clone();
        self.back = (self.back + 1) % self.capacity();
        self.size += 1;
    }

    /// Adds an element to the front of the queue.
    pub fn push_front(&mut self, value: T) {
        if self.size == self.capacity() {
            self.resize();
        }

        self.front = (self.front + self.capacity() - 1) % self.capacity();
        self.items[self.front] = value.clone();
        self.size += 1;
    }

    /// Removes and returns the element from the front of the queue.
    /// Returns `None` if the queue is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let value = self.items[self.front].clone();
        self.front = (self.front + 1) % self.capacity();
        self.size -= 1;

        Some(value)
    }

    /// Removes and returns the element from the front of the queue without checking for underflow.
    /// This is unsafe because it can cause undefined behavior if the queue is empty.
    pub fn pop_front_unchecked(&mut self) -> T {
        let value = self.items[self.front].clone();
        self.front = (self.front + 1) % self.capacity();
        self.size -= 1;

        value
    }

    /// Removes and returns the element from the back of the queue.
    /// Returns `None` if the queue is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.back = (self.back + self.capacity() - 1) % self.capacity();
        let value = self.items[self.back].clone();
        self.size -= 1;
        
        Some(value)
    }

    /// Removes and returns the element from the back of the queue without checking for underflow.
    /// This is unsafe because it can cause undefined behavior if the queue is empty.
    pub fn pop_back_unchecked(&mut self) -> T {
        self.back = (self.back + self.capacity() - 1) % self.capacity();
        let value = self.items[self.back].clone();
        self.size -= 1;

        value
    }

    fn resize(&mut self) {
        let new_size = self.capacity() * 2;
        let mut new_items = Vec::with_capacity(new_size);
        unsafe {
            new_items.set_len(new_size);
        }

        for i in 0..self.size {
            new_items[i] = self.items[(self.front + i) % self.capacity()].clone();
        }

        self.front = 0;
        self.back = self.size;
        self.items = new_items;
    }

    pub fn front(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        Some(self.items[self.front].clone())
    }

    pub fn front_unchecked(&self) -> T {
        self.items[self.front].clone()
    }

    pub fn back(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        Some(self.items[(self.back + self.capacity() - 1) % self.capacity()].clone())
    }

    pub fn back_unchecked(&self) -> T {
        self.items[(self.back + self.capacity() - 1) % self.capacity()].clone()
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }
}
