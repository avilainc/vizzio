//! Complex number types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.re, -self.im)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

impl Complex32 {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}
