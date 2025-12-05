//! Transform and coordinate utilities (Rust puro)

use crate::bim_core::Placement;

/// Utilitários de transformação
pub struct Transform;

impl Transform {
    /// Aplicar transformação a ponto
    pub fn apply_to_point(placement: &Placement, point: [f64; 3]) -> [f64; 3] {
        let m = &placement.matrix;

        let x = m[0] * point[0] + m[4] * point[1] + m[8] * point[2] + m[12];
        let y = m[1] * point[0] + m[5] * point[1] + m[9] * point[2] + m[13];
        let z = m[2] * point[0] + m[6] * point[1] + m[10] * point[2] + m[14];
        let w = m[3] * point[0] + m[7] * point[1] + m[11] * point[2] + m[15];

        if w.abs() > 1e-10 {
            [x / w, y / w, z / w]
        } else {
            [x, y, z]
        }
    }

    /// Aplicar transformação a vetor (sem translação)
    pub fn apply_to_vector(placement: &Placement, vector: [f64; 3]) -> [f64; 3] {
        let m = &placement.matrix;

        [
            m[0] * vector[0] + m[4] * vector[1] + m[8] * vector[2],
            m[1] * vector[0] + m[5] * vector[1] + m[9] * vector[2],
            m[2] * vector[0] + m[6] * vector[1] + m[10] * vector[2],
        ]
    }

    /// Criar transformação de rotação (ângulo em radianos)
    pub fn rotation_x(angle: f64) -> Placement {
        let c = angle.cos();
        let s = angle.sin();

        Placement {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, c, s, 0.0,
                0.0, -s, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_y(angle: f64) -> Placement {
        let c = angle.cos();
        let s = angle.sin();

        Placement {
            matrix: [
                c, 0.0, -s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                s, 0.0, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn rotation_z(angle: f64) -> Placement {
        let c = angle.cos();
        let s = angle.sin();

        Placement {
            matrix: [
                c, s, 0.0, 0.0,
                -s, c, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Criar transformação de escala
    pub fn scale(sx: f64, sy: f64, sz: f64) -> Placement {
        Placement {
            matrix: [
                sx, 0.0, 0.0, 0.0,
                0.0, sy, 0.0, 0.0,
                0.0, 0.0, sz, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Compor duas transformações (multiplicar matrizes)
    pub fn compose(a: &Placement, b: &Placement) -> Placement {
        let ma = &a.matrix;
        let mb = &b.matrix;
        let mut result = [0.0; 16];

        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += ma[row + k * 4] * mb[k + col * 4];
                }
                result[row + col * 4] = sum;
            }
        }

        Placement { matrix: result }
    }

    /// Inverter transformação (aproximação para transformações simples)
    pub fn inverse_simple(placement: &Placement) -> Option<Placement> {
        // Para transformações de rotação + translação simples
        let m = &placement.matrix;

        // Extrair rotação (3x3 superior esquerda)
        let r00 = m[0]; let r01 = m[4]; let r02 = m[8];
        let r10 = m[1]; let r11 = m[5]; let r12 = m[9];
        let r20 = m[2]; let r21 = m[6]; let r22 = m[10];

        // Transpor rotação (inversa para matrizes ortogonais)
        let rt00 = r00; let rt01 = r10; let rt02 = r20;
        let rt10 = r01; let rt11 = r11; let rt12 = r21;
        let rt20 = r02; let rt21 = r12; let rt22 = r22;

        // Translação original
        let tx = m[12];
        let ty = m[13];
        let tz = m[14];

        // Nova translação = -R^T * t
        let ntx = -(rt00 * tx + rt01 * ty + rt02 * tz);
        let nty = -(rt10 * tx + rt11 * ty + rt12 * tz);
        let ntz = -(rt20 * tx + rt21 * ty + rt22 * tz);

        Some(Placement {
            matrix: [
                rt00, rt10, rt20, 0.0,
                rt01, rt11, rt21, 0.0,
                rt02, rt12, rt22, 0.0,
                ntx, nty, ntz, 1.0,
            ],
        })
    }

    /// Decompor transformação em componentes
    pub fn decompose(placement: &Placement) -> (
        [f64; 3], // Translation
        [f64; 3], // Scale
        [f64; 3], // Rotation (Euler XYZ)
    ) {
        let m = &placement.matrix;

        // Translação
        let translation = [m[12], m[13], m[14]];

        // Escala (comprimento das colunas)
        let sx = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
        let sy = (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
        let sz = (m[8] * m[8] + m[9] * m[9] + m[10] * m[10]).sqrt();
        let scale = [sx, sy, sz];

        // Rotação (Euler angles - simplificado)
        let r11 = m[0] / sx;
        let r21 = m[1] / sx;
        let r31 = m[2] / sx;
        let r32 = m[6] / sz;
        let r33 = m[10] / sz;

        let rotation_y = (-r31).asin();
        let rotation_x = (r32 / rotation_y.cos()).atan2(r33 / rotation_y.cos());
        let rotation_z = (r21 / rotation_y.cos()).atan2(r11 / rotation_y.cos());

        let rotation = [rotation_x, rotation_y, rotation_z];

        (translation, scale, rotation)
    }

    /// Criar transformação look-at (câmera)
    pub fn look_at(eye: [f64; 3], target: [f64; 3], up: [f64; 3]) -> Placement {
        // Eixo Z (direção da câmera)
        let mut z = [
            eye[0] - target[0],
            eye[1] - target[1],
            eye[2] - target[2],
        ];
        let z_len = (z[0] * z[0] + z[1] * z[1] + z[2] * z[2]).sqrt();
        z[0] /= z_len;
        z[1] /= z_len;
        z[2] /= z_len;

        // Eixo X (cross(up, z))
        let mut x = [
            up[1] * z[2] - up[2] * z[1],
            up[2] * z[0] - up[0] * z[2],
            up[0] * z[1] - up[1] * z[0],
        ];
        let x_len = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        x[0] /= x_len;
        x[1] /= x_len;
        x[2] /= x_len;

        // Eixo Y (cross(z, x))
        let y = [
            z[1] * x[2] - z[2] * x[1],
            z[2] * x[0] - z[0] * x[2],
            z[0] * x[1] - z[1] * x[0],
        ];

        Placement {
            matrix: [
                x[0], y[0], z[0], 0.0,
                x[1], y[1], z[1], 0.0,
                x[2], y[2], z[2], 0.0,
                eye[0], eye[1], eye[2], 1.0,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_point() {
        let translation = Placement::from_translation(5.0, 10.0, 15.0);
        let point = [1.0, 2.0, 3.0];

        let transformed = Transform::apply_to_point(&translation, point);
        assert_eq!(transformed, [6.0, 12.0, 18.0]);
    }

    #[test]
    fn test_rotation_z() {
        use std::f64::consts::PI;

        let rotation = Transform::rotation_z(PI / 2.0); // 90 graus
        let point = [1.0, 0.0, 0.0];

        let rotated = Transform::apply_to_point(&rotation, point);

        // Após rotação de 90° em Z, (1,0,0) → (0,1,0)
        assert!((rotated[0] - 0.0).abs() < 1e-10);
        assert!((rotated[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_compose() {
        let t1 = Placement::from_translation(1.0, 0.0, 0.0);
        let t2 = Placement::from_translation(0.0, 2.0, 0.0);

        let composed = Transform::compose(&t1, &t2);
        let point = [0.0, 0.0, 0.0];

        let result = Transform::apply_to_point(&composed, point);
        assert_eq!(result, [1.0, 2.0, 0.0]);
    }
}
