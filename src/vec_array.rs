#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VecArray<T: Default + Copy + Clone, const SIZE: usize> {
    array: [T; SIZE],
    capacity: usize,
    index: usize,
}

pub struct VecArrayI<T: Default + Copy + Clone, const SIZE: usize> {
    vec_array: VecArray<T, SIZE>,
    index: usize,
}

impl<T: Default + Copy + Clone, const SIZE: usize> IntoIterator for VecArray<T, SIZE> {
    type Item = T;
    type IntoIter = VecArrayI<T, SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        VecArrayI {
            vec_array: self,
            index: 0,
        }
    }
}

impl<T: Default + Copy + Clone, const SIZE: usize> Iterator for VecArrayI<T, SIZE> {
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

impl<T: Default + Copy + Clone, const SIZE: usize> VecArray<T, SIZE> {
    pub fn new() -> VecArray<T, SIZE> {
        VecArray {
            array: [Default::default(); SIZE],
            capacity: 0,
            index: SIZE,
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
}
