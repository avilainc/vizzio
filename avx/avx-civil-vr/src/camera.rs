use avila_vec3d::{Vec3, Mat4};
use std::f32::consts::PI;

pub struct VRCamera {
    pub position: Vec3,
    pub yaw: f32,   // Horizontal rotation
    pub pitch: f32, // Vertical rotation
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub move_speed: f32,
    pub look_sensitivity: f32,
}

impl VRCamera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 1.7, 0.0), // Eye height
            yaw: 0.0,
            pitch: 0.0,
            fov: PI / 3.0, // 60 degrees
            aspect_ratio: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            move_speed: 5.0,
            look_sensitivity: 0.002,
        }
    }

    pub fn set_position(&mut self, position: [f64; 3]) {
        self.position = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    }

    pub fn update(&mut self, delta_time: f32, forward: bool, backward: bool, left: bool, right: bool) {
        let mut move_dir = Vec3::ZERO;

        if forward { move_dir.z -= 1.0; }
        if backward { move_dir.z += 1.0; }
        if left { move_dir.x -= 1.0; }
        if right { move_dir.x += 1.0; }

        if move_dir != Vec3::ZERO {
            move_dir = move_dir.normalize().unwrap_or(Vec3::Z);
            // Rotate movement direction by yaw
            let cos_yaw = self.yaw.cos();
            let sin_yaw = self.yaw.sin();
            let rotated_x = move_dir.x * cos_yaw - move_dir.z * sin_yaw;
            let rotated_z = move_dir.x * sin_yaw + move_dir.z * cos_yaw;

            self.position.x += rotated_x * self.move_speed * delta_time;
            self.position.z += rotated_z * self.move_speed * delta_time;
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x * self.look_sensitivity;
        self.pitch -= delta_y * self.look_sensitivity;

        // Clamp pitch to prevent flipping
        self.pitch = self.pitch.clamp(-PI/2.0 + 0.1, PI/2.0 - 0.1);
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        // Simple translation for now
        Mat4::translation(-self.position)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        // Simple orthographic for now
        Mat4::IDENTITY
    }

    pub fn get_view_projection_matrix(&self) -> [[f32; 4]; 4] {
        // For now, return identity
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }
}