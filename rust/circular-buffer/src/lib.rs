use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data.len() == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.data.push_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data.len() == 0 {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.data.pop_front().unwrap())
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(element)
    }
}
