//! Advanced camera systems with DoF and motion blur

/// Simple ray structure
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Ray origin
    pub origin: [f64; 3],
    /// Ray direction (normalized)
    pub direction: [f64; 3],
}

/// Camera with depth of field and motion blur
#[derive(Debug, Clone)]
pub struct AdvancedCamera {
    /// Camera position
    pub position: [f64; 3],
    /// Forward direction (normalized)
    pub forward: [f64; 3],
    /// Right direction (normalized)
    pub right: [f64; 3],
    /// Up direction (normalized)
    pub up: [f64; 3],
    /// Vertical FOV in degrees
    pub fov: f64,
    /// Aspect ratio (width / height)
    pub aspect: f64,
    /// Aperture diameter for depth of field
    pub aperture: f64,
    /// Focus distance for depth of field
    pub focus_distance: f64,
    /// Motion blur velocity
    pub velocity: [f64; 3],
    /// Exposure time for motion blur
    pub exposure_time: f64,
}

impl AdvancedCamera {
    /// Create a new advanced camera
    pub fn new(
        position: [f64; 3],
        target: [f64; 3],
        up: [f64; 3],
        fov: f64,
        aspect: f64,
    ) -> Self {
        let forward = Self::normalize(Self::sub(target, position));
        let right = Self::normalize(Self::cross(forward, up));
        let up = Self::normalize(Self::cross(right, forward));

        Self {
            position,
            forward,
            right,
            up,
            fov,
            aspect,
            aperture: 0.0,
            focus_distance: 1.0,
            velocity: [0.0; 3],
            exposure_time: 0.0,
        }
    }

    /// Set depth of field parameters
    pub fn with_dof(mut self, aperture: f64, focus_distance: f64) -> Self {
        self.aperture = aperture;
        self.focus_distance = focus_distance;
        self
    }

    /// Set motion blur parameters
    pub fn with_motion_blur(mut self, velocity: [f64; 3], exposure_time: f64) -> Self {
        self.velocity = velocity;
        self.exposure_time = exposure_time;
        self
    }

    /// Generate a ray for the given normalized screen coordinates (0-1)
    pub fn generate_ray(&self, x: f64, y: f64) -> Ray {
        let fov_rad = self.fov * std::f64::consts::PI / 180.0;
        let tan_half_fov = (fov_rad / 2.0).tan();

        let height = 2.0 * tan_half_fov;
        let width = height * self.aspect;

        // Normalized device coordinates
        let ndc_x = (x - 0.5) * width;
        let ndc_y = (0.5 - y) * height;

        let mut dir = Self::add(
            Self::add(
                Self::scale(self.right, ndc_x),
                Self::scale(self.up, ndc_y),
            ),
            self.forward,
        );
        dir = Self::normalize(dir);

        let mut origin = self.position;

        // Apply depth of field
        if self.aperture > 0.0 {
            let (dx, dy) = Self::random_in_disk();
            let offset = Self::add(
                Self::scale(self.right, dx * self.aperture),
                Self::scale(self.up, dy * self.aperture),
            );
            origin = Self::add(origin, offset);

            // Adjust direction for focus plane
            let focus_dist = self.focus_distance;
            let focus_point = Self::add(
                self.position,
                Self::scale(dir, focus_dist),
            );
            dir = Self::normalize(Self::sub(focus_point, origin));
        }

        // Apply motion blur
        let time = if self.exposure_time > 0.0 {
            rand::random::<f64>() * self.exposure_time
        } else {
            0.0
        };
        origin = Self::add(origin, Self::scale(self.velocity, time));

        Ray { origin, direction: dir }
    }

    // Helper functions
    fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
    }

    fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
        [v[0] * s, v[1] * s, v[2] * s]
    }

    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn length(v: [f64; 3]) -> f64 {
        Self::dot(v, v).sqrt()
    }

    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = Self::length(v);
        if len > 0.0 {
            Self::scale(v, 1.0 / len)
        } else {
            v
        }
    }

    fn random_in_disk() -> (f64, f64) {
        use std::f64::consts::PI;
        let r = rand::random::<f64>().sqrt();
        let theta = 2.0 * PI * rand::random::<f64>();
        (r * theta.cos(), r * theta.sin())
    }
}

/// Panoramic camera
#[derive(Debug, Clone)]
pub struct PanoramicCamera {
    position: [f64; 3],
    fov_horizontal: f64,
    fov_vertical: f64,
}

impl PanoramicCamera {
    pub fn new(position: [f64; 3], fov_h: f64, fov_v: f64) -> Self {
        Self {
            position,
            fov_horizontal: fov_h,
            fov_vertical: fov_v,
        }
    }

    /// Generate ray for panoramic projection
    pub fn generate_ray(&self, theta: f64, phi: f64) -> Ray {
        let x = theta.sin() * phi.cos();
        let y = phi.sin();
        let z = theta.cos() * phi.cos();

        Ray {
            origin: self.position,
            direction: [x, y, z],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_camera_creation() {
        let cam = AdvancedCamera::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0],
            60.0,
            16.0 / 9.0,
        );
        assert_eq!(cam.position, [0.0, 0.0, 0.0]);
        assert_eq!(cam.fov, 60.0);
    }

    #[test]
    fn test_ray_generation() {
        let cam = AdvancedCamera::new(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            45.0,
            1.0,
        );
        let ray = cam.generate_ray(0.5, 0.5);
        assert_eq!(ray.origin, [0.0, 0.0, 5.0]);
    }

    #[test]
    fn test_dof_settings() {
        let cam = AdvancedCamera::new(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0],
            60.0,
            1.0,
        )
        .with_dof(0.1, 5.0);

        assert_eq!(cam.aperture, 0.1);
        assert_eq!(cam.focus_distance, 5.0);
    }

    #[test]
    fn test_panoramic_camera() {
        let cam = PanoramicCamera::new([0.0, 0.0, 0.0], 360.0, 180.0);
        let ray = cam.generate_ray(0.0, 0.0);
        assert_eq!(ray.origin, [0.0, 0.0, 0.0]);
    }
}
