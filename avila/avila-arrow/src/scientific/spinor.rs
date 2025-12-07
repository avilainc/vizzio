//! Spinor types for quantum mechanics

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spinor {
    pub up: [f64; 2],   // Complex number as [real, imag]
    pub down: [f64; 2], // Complex number as [real, imag]
}

impl Spinor {
    pub fn new(up_re: f64, up_im: f64, down_re: f64, down_im: f64) -> Self {
        Self {
            up: [up_re, up_im],
            down: [down_re, down_im],
        }
    }

    pub fn spin_up() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn spin_down() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }

    pub fn normalize(&self) -> Self {
        let norm = ((self.up[0] * self.up[0] + self.up[1] * self.up[1]) +
                   (self.down[0] * self.down[0] + self.down[1] * self.down[1])).sqrt();
        Self {
            up: [self.up[0] / norm, self.up[1] / norm],
            down: [self.down[0] / norm, self.down[1] / norm],
        }
    }
}
