use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

/// A fixed-length FIFO queue.
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Fifo<T, const LENGTH: usize> {
    array: [T; LENGTH],
    position: usize,
}

impl<T, const LENGTH: usize> Default for Fifo<T, LENGTH>
where
    T: Default + Copy,
{
    fn default() -> Self {
        if LENGTH == 0 {
            near_sdk::env::panic(b"Zero-length FIFO queue are not supported");
        }
        Self {
            array: [T::default(); LENGTH],
            position: 0,
        }
    }
}

impl<T, const LENGTH: usize> Fifo<T, LENGTH> {
    /// Creates a queue of size `LENGTH`, filled with `T::default()`.
    pub fn new() -> Self
    where
        T: Default + Copy,
    {
        Self::default()
    }

    /// Adds an item to the queue.
    pub fn push(&mut self, item: T) {
        let insert_position = (self.position + LENGTH) % LENGTH;
        self.position = (self.position + 1) % LENGTH;
        self.array[insert_position] = item;
    }

    /// Returns an iterator over the queued items.
    pub fn iter(&self) -> FifoIterator<'_, T, LENGTH> {
        FifoIterator {
            array: &self.array,
            start_position: self.position,
            count: LENGTH,
        }
    }
}

impl<'a, T: 'a, const LENGTH: usize> IntoIterator for &'a Fifo<T, LENGTH> {
    type Item = &'a T;

    type IntoIter = FifoIterator<'a, T, LENGTH>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the queued values.
pub struct FifoIterator<'a, T, const LENGTH: usize> {
    array: &'a [T; LENGTH],
    start_position: usize,
    count: usize,
}

impl<'a, T: 'a, const LENGTH: usize> Iterator for FifoIterator<'a, T, LENGTH> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count.checked_sub(1)?;
        let item = &self.array[self.start_position % LENGTH];
        self.start_position += 1;
        Some(item)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        let mut queue = Fifo::<_, 5>::new();
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0]
        );

        queue.push(100);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 100]
        );

        queue.push(200);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 0, 100, 200]
        );

        queue.push(300);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 0, 100, 200, 300]
        );

        queue.push(400);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![0, 100, 200, 300, 400]
        );

        queue.push(500);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![100, 200, 300, 400, 500]
        );
    }
}
