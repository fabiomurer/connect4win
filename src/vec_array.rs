use std::ops::{Index, IndexMut};

#[macro_export]
macro_rules! VecArray {
    ($x:expr, $t:ty) => {
        {
            let mut v: VecArray<$t, $x.len()> = VecArray::new();
            for i in $x {
                v.push(i);
            }
            v
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VecArray<T: Default + Copy + Clone + Ord, const SIZE: usize> {
    array: [T; SIZE],
    capacity: usize,
    index: usize,
}

pub struct VecArrayI<T: Default + Copy + Clone + Ord, const SIZE: usize> {
    vec_array: VecArray<T, SIZE>,
    index: usize,
}

impl<T: Default + Copy + Clone + Ord, const SIZE: usize> IntoIterator for VecArray<T, SIZE> {
    type Item = T;
    type IntoIter = VecArrayI<T, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        VecArrayI {
            vec_array: self,
            index: 0,
        }
    }
}

impl<T: Default + Copy + Clone + Ord, const SIZE: usize> Iterator for VecArrayI<T, SIZE> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec_array.index {
            let res = Some(self.vec_array.array[self.index]);
            self.index += 1;
            res
        } else {
            None
        }
    }
}

impl<T: Default + Copy + Clone + Ord, const SIZE: usize> Index<usize> for VecArray<T, SIZE> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.array[0..self.index][index]
    }
}

impl<T: Default + Copy + Clone + Ord, const SIZE: usize> IndexMut<usize> for VecArray<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[0..self.index][index]
    }
}

impl<T: Default + Copy + Clone + Ord, const SIZE: usize> VecArray<T, SIZE> {
    pub fn new() -> VecArray<T, SIZE> {
        VecArray {
            array: [Default::default(); SIZE],
            capacity: SIZE,
            index: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.array[self.index] = value;
        self.index += 1;
    }

    pub fn pop(&mut self) -> T {
        self.index -= 1;
        self.array[self.index]
    }

    pub fn len(&self) -> usize {
        self.index
    }

    pub fn sort(&mut self) {
        self.array[0..self.index].sort();
    }

    pub fn reverse(&mut self) {
        self.array[0..self.index].reverse();
    }

    pub fn get_array(&self) -> [T; SIZE] {
        self.array
    }
}
