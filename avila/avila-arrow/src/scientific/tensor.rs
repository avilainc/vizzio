//! Tensor types (Tensor4D and multi-dimensional arrays)

#[derive(Debug, Clone)]
pub struct Tensor4D<T> {
    data: Vec<T>,
    shape: [usize; 4],
}

impl<T: Clone> Tensor4D<T> {
    pub fn new(shape: [usize; 4], default: T) -> Self {
        let size = shape[0] * shape[1] * shape[2] * shape[3];
        Self {
            data: vec![default; size],
            shape,
        }
    }

    pub fn shape(&self) -> [usize; 4] {
        self.shape
    }

    pub fn get(&self, i: usize, j: usize, k: usize, l: usize) -> Option<&T> {
        if i >= self.shape[0] || j >= self.shape[1] || k >= self.shape[2] || l >= self.shape[3] {
            return None;
        }
        let index = ((i * self.shape[1] + j) * self.shape[2] + k) * self.shape[3] + l;
        self.data.get(index)
    }

    pub fn set(&mut self, i: usize, j: usize, k: usize, l: usize, value: T) {
        if i >= self.shape[0] || j >= self.shape[1] || k >= self.shape[2] || l >= self.shape[3] {
            return;
        }
        let index = ((i * self.shape[1] + j) * self.shape[2] + k) * self.shape[3] + l;
        if index < self.data.len() {
            self.data[index] = value;
        }
    }
}
