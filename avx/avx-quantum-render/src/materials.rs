//! Advanced materials and BSDF (Bidirectional Scattering Distribution Function)

use std::f64::consts::PI;

/// Material properties for physically-based rendering
#[derive(Debug, Clone, Copy)]
pub struct MaterialProperties {
    /// Metallic factor [0, 1]
    pub metallic: f64,
    /// Roughness [0, 1]
    pub roughness: f64,
    /// Specular intensity [0, 1]
    pub specular: f64,
    /// IOR (Index of Refraction)
    pub ior: f64,
    /// Subsurface scattering
    pub subsurface: f64,
    /// Clearcoat layer
    pub clearcoat: f64,
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            metallic: 0.0,
            roughness: 0.5,
            specular: 0.5,
            ior: 1.5,
            subsurface: 0.0,
            clearcoat: 0.0,
        }
    }
}

impl MaterialProperties {
    /// Dielectric material (glass, plastic)
    pub fn dielectric(ior: f64, roughness: f64) -> Self {
        Self {
            metallic: 0.0,
            roughness,
            specular: 0.5,
            ior,
            subsurface: 0.0,
            clearcoat: 0.0,
        }
    }

    /// Metal material (gold, copper, aluminum)
    pub fn metal(roughness: f64) -> Self {
        Self {
            metallic: 1.0,
            roughness,
            specular: 1.0,
            ior: 2.0,
            subsurface: 0.0,
            clearcoat: 0.0,
        }
    }

    /// Cloth material
    pub fn cloth(roughness: f64) -> Self {
        Self {
            metallic: 0.0,
            roughness,
            specular: 0.1,
            ior: 1.5,
            subsurface: 0.0,
            clearcoat: 0.0,
        }
    }

    /// Skin material
    pub fn skin() -> Self {
        Self {
            metallic: 0.0,
            roughness: 0.4,
            specular: 0.4,
            ior: 1.4,
            subsurface: 0.5,
            clearcoat: 0.0,
        }
    }
}

/// Fresnel equations for reflection
pub struct Fresnel;

impl Fresnel {
    /// Schlick approximation for Fresnel reflectance
    pub fn schlick(cos_theta: f64, ior: f64) -> f64 {
        let f0 = ((ior - 1.0) / (ior + 1.0)).powi(2);
        f0 + (1.0 - f0) * (1.0 - cos_theta).powi(5)
    }

    /// Exact Fresnel for dielectrics
    pub fn dielectric(cos_theta_i: f64, ior1: f64, ior2: f64) -> f64 {
        let eta = ior1 / ior2;
        let cos_theta_t_sq = 1.0 - eta * eta * (1.0 - cos_theta_i * cos_theta_i);

        if cos_theta_t_sq < 0.0 {
            // Total internal reflection
            return 1.0;
        }

        let cos_theta_t = cos_theta_t_sq.sqrt();
        let r_s = (ior1 * cos_theta_i - ior2 * cos_theta_t) /
                  (ior1 * cos_theta_i + ior2 * cos_theta_t);
        let r_p = (ior2 * cos_theta_i - ior1 * cos_theta_t) /
                  (ior2 * cos_theta_i + ior1 * cos_theta_t);

        (r_s * r_s + r_p * r_p) / 2.0
    }
}

/// GGX microfacet distribution
pub struct GGX;

impl GGX {
    /// GGX distribution function
    pub fn d(cos_theta: f64, roughness: f64) -> f64 {
        let a = roughness * roughness;
        let cos2 = cos_theta * cos_theta;
        let denom = PI * (cos2 * (a * a - 1.0) + 1.0).powi(2);
        (a * a) / denom.max(1e-6)
    }

    /// GGX geometry function (Smith approximation)
    pub fn g(cos_theta: f64, roughness: f64) -> f64 {
        let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
        cos_theta / (cos_theta * (1.0 - k) + k).max(1e-6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_properties() {
        let mat = MaterialProperties::metal(0.5);
        assert_eq!(mat.metallic, 1.0);
    }

    #[test]
    fn test_fresnel_schlick() {
        let f = Fresnel::schlick(0.5, 1.5);
        assert!(f >= 0.0 && f <= 1.0);
    }

    #[test]
    fn test_ggx_distribution() {
        let d = GGX::d(0.8, 0.5);
        assert!(d >= 0.0);
    }
}
