//! A generic bounded queue (ring buffer) implementation.
//!
//! This module provides a fixed-capacity queue that uses a circular buffer
//! internally for efficient push and pop operations.

use std::fmt;

/// Error type for queue operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The queue is full and cannot accept more items.
    Full,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Full => write!(f, "queue is full"),
        }
    }
}

impl std::error::Error for Error {}

/// A bounded queue (ring buffer) with fixed capacity.
///
/// The queue stores elements in a circular buffer, allowing O(1) push and pop
/// operations without memory reallocation.
#[derive(Debug)]
pub struct Queue<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize,
}

impl<T> Queue<T> {
    /// Creates a new queue with the specified capacity.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");

        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || None);

        Self {
            buffer,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Adds an item to the back of the queue.
    ///
    /// Returns `Err(Error::Full)` if the queue is at capacity.
    pub fn push(&mut self, item: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::Full);
        }

        self.buffer[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity();
        self.len += 1;

        Ok(())
    }

    /// Removes and returns the item at the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;

        item
    }

    /// Returns the number of items in the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the queue contains no items.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the queue is at capacity.
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Returns the capacity of the queue.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop_single_item() {
        let mut queue: Queue<i32> = Queue::new(3);

        queue.push(42).unwrap();
        assert_eq!(queue.len(), 1);

        let item = queue.pop();
        assert_eq!(item, Some(42));
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn push_and_pop_multiple_items() {
        let mut queue = Queue::new(3);

        queue.push(1).unwrap();
        queue.push(2).unwrap();
        queue.push(3).unwrap();

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
    }

    #[test]
    fn fifo_ordering() {
        let mut queue = Queue::new(5);

        for i in 0..5 {
            queue.push(i).unwrap();
        }

        for i in 0..5 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn full_queue_rejects_push() {
        let mut queue = Queue::new(2);

        queue.push(1).unwrap();
        queue.push(2).unwrap();

        let result = queue.push(3);
        assert_eq!(result, Err(Error::Full));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn is_full_returns_true_at_capacity() {
        let mut queue = Queue::new(2);

        assert!(!queue.is_full());
        queue.push(1).unwrap();
        assert!(!queue.is_full());
        queue.push(2).unwrap();
        assert!(queue.is_full());
    }

    #[test]
    fn empty_queue_returns_none() {
        let mut queue: Queue<i32> = Queue::new(3);

        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn is_empty_returns_true_when_empty() {
        let mut queue = Queue::new(2);

        assert!(queue.is_empty());
        queue.push(1).unwrap();
        assert!(!queue.is_empty());
        queue.pop();
        assert!(queue.is_empty());
    }

    #[test]
    fn wraparound_behavior() {
        let mut queue = Queue::new(3);

        // Fill the queue
        queue.push(1).unwrap();
        queue.push(2).unwrap();
        queue.push(3).unwrap();

        // Remove two items, head advances
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        // Add two more items, tail wraps around
        queue.push(4).unwrap();
        queue.push(5).unwrap();

        // Verify correct order after wraparound
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(5));
        assert!(queue.is_empty());
    }

    #[test]
    fn multiple_wraparound_cycles() {
        let mut queue = Queue::new(2);

        for cycle in 0..5 {
            let base = cycle * 2;

            queue.push(base).unwrap();
            queue.push(base + 1).unwrap();
            assert!(queue.is_full());

            assert_eq!(queue.pop(), Some(base));
            assert_eq!(queue.pop(), Some(base + 1));
            assert!(queue.is_empty());
        }
    }

    #[test]
    fn capacity_of_one() {
        let mut queue = Queue::new(1);

        assert!(queue.is_empty());
        queue.push(42).unwrap();
        assert!(queue.is_full());
        assert_eq!(queue.push(99), Err(Error::Full));
        assert_eq!(queue.pop(), Some(42));
        assert!(queue.is_empty());
    }

    #[test]
    #[should_panic(expected = "capacity must be greater than zero")]
    fn zero_capacity_panics() {
        let _queue: Queue<i32> = Queue::new(0);
    }

    #[test]
    fn works_with_non_copy_types() {
        let mut queue = Queue::new(2);

        queue.push(String::from("hello")).unwrap();
        queue.push(String::from("world")).unwrap();

        assert_eq!(queue.pop(), Some(String::from("hello")));
        assert_eq!(queue.pop(), Some(String::from("world")));
    }
}
