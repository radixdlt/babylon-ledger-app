#[repr(C, align(4))]
pub struct StaticVec<T, const N: usize> {
    length: usize,
    data: [T; N],
}

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

impl<T: Copy, const N: usize> StaticVec<T, N> {
    pub const fn new(initial_value: T) -> Self {
        Self {
            length: 0,
            data: [initial_value; N],
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) {
        if self.length == N {
            return;
        }
        self.data[self.length] = item;
        self.length += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        Some(self.data[self.length])
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn insert(&mut self, position: usize, item: T) {
        if self.length == N {
            return;
        }

        if position > self.length {
            return;
        }

        for i in (position..self.length).rev() {
            self.data[i + 1] = self.data[i];
        }
        self.data[position] = item;
        self.length += 1;
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        for item in other {
            self.push(*item);
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.length = 0;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data[0..self.length]
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[0..self.length]
    }

    #[inline]
    pub fn as_mut_buffer(&mut self) -> &mut [T] {
        &mut self.data
    }

    #[inline]
    pub fn is_not_empty(&self) -> bool {
        self.length != 0
    }

    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.length == 0 {
            return None;
        }
        Some(&mut self.data[self.length - 1])
    }

    #[inline]
    pub fn first(&self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        Some(self.data[0])
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }
        Some(self.data[index])
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }
        let item = self.data[index];
        for i in index..self.length - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.length -= 1;
        Some(item)
    }
}

impl<T, const N: usize> AsSlice<T> for StaticVec<T, N> {
    #[inline]
    fn as_slice(&self) -> &[T] {
        &self.data[0..self.length]
    }
}
