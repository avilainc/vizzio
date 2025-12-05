//! Testes de escalabilidade

pub struct ScalabilityTest {
    dataset_sizes: Vec<usize>,
}

impl ScalabilityTest {
    pub fn new(sizes: Vec<usize>) -> Self {
        Self {
            dataset_sizes: sizes,
        }
    }

    pub fn run_test<F>(&self, algorithm: F) -> Vec<(usize, f64)>
    where
        F: Fn(usize) -> f64,
    {
        self.dataset_sizes
            .iter()
            .map(|&size| (size, algorithm(size)))
            .collect()
    }
}
