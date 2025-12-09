//! Câmera 3D para visualização

use alloc::vec::Vec;

/// Câmera perspectiva
#[derive(Debug, Clone)]
pub struct Camera {
    /// Posição (x, y, z)
    pub position: [f32; 3],
    /// Alvo que a câmera olha
    pub target: [f32; 3],
    /// Vetor "up"
    pub up: [f32; 3],
    /// Campo de visão (FOV) em graus
    pub fov: f32,
    /// Aspect ratio (width/height)
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// View matrix (para WebXR)
    pub view_matrix: [f32; 16],
    /// Projection matrix (para WebXR)
    pub projection_matrix: [f32; 16],
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [5.0, 5.0, 5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 45.0,
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            view_matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
            projection_matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Camera {
    /// Cria matriz de view
    pub fn get_view_matrix(&self) -> [f32; 16] {
        look_at(&self.position, &self.target, &self.up)
    }

    /// Cria matriz de projeção perspectiva
    pub fn get_projection_matrix(&self) -> [f32; 16] {
        perspective(self.fov, self.aspect, self.near, self.far)
    }

    /// Obtém view matrix (usa WebXR se disponível, senão calcula)
    pub fn view_matrix(&self) -> [f32; 16] {
        // Se view_matrix é identidade, calcular
        if self.view_matrix[0] == 1.0 && self.view_matrix[5] == 1.0 && self.view_matrix[10] == 1.0 {
            self.get_view_matrix()
        } else {
            self.view_matrix
        }
    }

    /// Obtém projection matrix (usa WebXR se disponível, senão calcula)
    pub fn projection_matrix(&self) -> [f32; 16] {
        // Se projection_matrix é identidade, calcular
        if self.projection_matrix[0] == 1.0 && self.projection_matrix[5] == 1.0 && self.projection_matrix[10] == 1.0 {
            self.get_projection_matrix()
        } else {
            self.projection_matrix
        }
    }

    /// Orbita ao redor do alvo
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calcula distância atual
        let dx = self.position[0] - self.target[0];
        let dy = self.position[1] - self.target[1];
        let dz = self.position[2] - self.target[2];
        let radius = (dx * dx + dy * dy + dz * dz).sqrt();

        // Calcula ângulos atuais
        let mut yaw = dz.atan2(dx);
        let mut pitch = (dy / radius).asin();

        // Aplica deltas
        yaw += delta_yaw;
        pitch = (pitch + delta_pitch).clamp(-1.5, 1.5);

        // Nova posição
        self.position[0] = self.target[0] + radius * pitch.cos() * yaw.cos();
        self.position[1] = self.target[1] + radius * pitch.sin();
        self.position[2] = self.target[2] + radius * pitch.cos() * yaw.sin();
    }

    /// Zoom in/out
    pub fn zoom(&mut self, delta: f32) {
        let dir_x = self.target[0] - self.position[0];
        let dir_y = self.target[1] - self.position[1];
        let dir_z = self.target[2] - self.position[2];

        let len = (dir_x * dir_x + dir_y * dir_y + dir_z * dir_z).sqrt();
        if len > 0.1 {
            let norm_x = dir_x / len;
            let norm_y = dir_y / len;
            let norm_z = dir_z / len;

            self.position[0] += norm_x * delta;
            self.position[1] += norm_y * delta;
            self.position[2] += norm_z * delta;
        }
    }
}

/// Cria matriz look-at
fn look_at(eye: &[f32; 3], target: &[f32; 3], up: &[f32; 3]) -> [f32; 16] {
    let mut z = [
        eye[0] - target[0],
        eye[1] - target[1],
        eye[2] - target[2],
    ];
    let z_len = (z[0] * z[0] + z[1] * z[1] + z[2] * z[2]).sqrt();
    z[0] /= z_len;
    z[1] /= z_len;
    z[2] /= z_len;

    let mut x = [
        up[1] * z[2] - up[2] * z[1],
        up[2] * z[0] - up[0] * z[2],
        up[0] * z[1] - up[1] * z[0],
    ];
    let x_len = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
    x[0] /= x_len;
    x[1] /= x_len;
    x[2] /= x_len;

    let y = [
        z[1] * x[2] - z[2] * x[1],
        z[2] * x[0] - z[0] * x[2],
        z[0] * x[1] - z[1] * x[0],
    ];

    [
        x[0], y[0], z[0], 0.0,
        x[1], y[1], z[1], 0.0,
        x[2], y[2], z[2], 0.0,
        -x[0] * eye[0] - x[1] * eye[1] - x[2] * eye[2],
        -y[0] * eye[0] - y[1] * eye[1] - y[2] * eye[2],
        -z[0] * eye[0] - z[1] * eye[1] - z[2] * eye[2],
        1.0,
    ]
}

/// Cria matriz de projeção perspectiva
fn perspective(fov_degrees: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let fov = fov_degrees * core::f32::consts::PI / 180.0;
    let f = 1.0 / (fov / 2.0).tan();

    [
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (far + near) / (near - far), -1.0,
        0.0, 0.0, (2.0 * far * near) / (near - far), 0.0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let cam = Camera::default();
        assert_eq!(cam.fov, 45.0);
    }

    #[test]
    fn test_camera_matrices() {
        let cam = Camera::default();
        let view = cam.view_matrix();
        let proj = cam.projection_matrix();

        assert_eq!(view.len(), 16);
        assert_eq!(proj.len(), 16);
    }
}
