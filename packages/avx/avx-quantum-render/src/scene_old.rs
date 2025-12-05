//! Cena 3D para renderização quântica

use crate::photon::Vertex;

const EPSILON: f64 = 1e-6;

#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn scale(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn length(v: [f64; 3]) -> f64 {
    dot(v, v).sqrt()
}

#[inline]
fn normalize_or(v: [f64; 3], fallback: [f64; 3]) -> [f64; 3] {
    let len = length(v);
    if len > EPSILON {
        scale(v, 1.0 / len)
    } else {
        fallback
    }
}

#[inline]
fn normalize(v: [f64; 3]) -> [f64; 3] {
    normalize_or(v, [0.0, 1.0, 0.0])
}

#[inline]
fn negate(v: [f64; 3]) -> [f64; 3] {
    [-v[0], -v[1], -v[2]]
}

#[inline]
fn orient_normal(normal: [f64; 3], direction: [f64; 3]) -> [f64; 3] {
    if dot(normal, direction) > 0.0 {
        negate(normal)
    } else {
        normal
    }
}

/// Material de superfície
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    /// Lambertiano (difuso ideal)
    Lambertian {
        /// Albedo (refletividade difusa) [0, 1]
        albedo: f64,
    },

    /// Especular (espelho)
    Specular {
        /// Refletância especular [0, 1]
        reflectance: f64,
    },

    /// Dielétrico (vidro, água)
    Dielectric {
        /// Índice de refração (n)
        refractive_index: f64,
        /// Transmitância [0, 1]
        transmittance: f64,
    },

    /// Metal (condutor)
    Metal {
        /// Refletância [0, 1]
        reflectance: f64,
        /// Rugosidade [0, 1]
        roughness: f64,
    },

    /// Absorvente (corpo negro)
    Absorbing {
        /// Coeficiente de absorção (m⁻¹)
        absorption_coeff: f64,
    },
}

impl Material {
    /// Cria material Lambertiano
    pub fn lambertian(albedo: f64) -> Self {
        Material::Lambertian {
            albedo: albedo.clamp(0.0, 1.0),
        }
    }

    /// Cria espelho perfeito
    pub fn mirror() -> Self {
        Material::Specular { reflectance: 1.0 }
    }

    /// Cria vidro (n ≈ 1.5)
    pub fn glass() -> Self {
        Material::Dielectric {
            refractive_index: 1.5,
            transmittance: 0.95,
        }
    }

    /// Cria metal
    pub fn metal(reflectance: f64) -> Self {
        Material::Metal {
            reflectance: reflectance.clamp(0.0, 1.0),
            roughness: 0.1,
        }
    }

    /// Retorna índice de refração (se aplicável)
    pub fn refractive_index(&self) -> f64 {
        match self {
            Material::Dielectric {
                refractive_index, ..
            } => *refractive_index,
            _ => 1.0,
        }
    }

    /// Retorna albedo/reflectância
    pub fn reflectance(&self) -> f64 {
        match self {
            Material::Lambertian { albedo } => *albedo,
            Material::Specular { reflectance } => *reflectance,
            Material::Metal { reflectance, .. } => *reflectance,
            Material::Dielectric { transmittance, .. } => 1.0 - transmittance,
            Material::Absorbing { .. } => 0.0,
        }
    }
}

/// Registro de interseção raio-superfície
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    /// Ponto de impacto
    pub point: [f64; 3],
    /// Normal no ponto (orientada contra o raio incidente)
    pub normal: [f64; 3],
    /// Distância ao longo do raio
    pub distance: f64,
}

/// Triângulo individual
#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: [f64; 3],
    pub v1: [f64; 3],
    pub v2: [f64; 3],
    pub normal: [f64; 3],
}

impl Triangle {
    /// Cria triângulo a partir dos vértices
    pub fn new(v0: [f64; 3], v1: [f64; 3], v2: [f64; 3]) -> Self {
        let edge1 = sub(v1, v0);
        let edge2 = sub(v2, v0);
        let normal = normalize_or(cross(edge1, edge2), [0.0, 1.0, 0.0]);

        Self { v0, v1, v2, normal }
    }

    /// Interseção usando o algoritmo de Möller–Trumbore
    pub fn intersect(&self, origin: [f64; 3], direction: [f64; 3]) -> Option<HitRecord> {
        let edge1 = sub(self.v1, self.v0);
        let edge2 = sub(self.v2, self.v0);
        let pvec = cross(direction, edge2);
        let det = dot(edge1, pvec);

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = sub(origin, self.v0);
        let u = dot(tvec, pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = cross(tvec, edge1);
        let v = dot(direction, qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = dot(edge2, qvec) * inv_det;
        if t <= EPSILON {
            return None;
        }

        let point = add(origin, scale(direction, t));
        let normal = if det < 0.0 {
            negate(self.normal)
        } else {
            self.normal
        };

        Some(HitRecord {
            point,
            normal: orient_normal(normalize(normal), direction),
            distance: t,
        })
    }
}

/// Tipos geométricos suportados
#[derive(Debug, Clone)]
pub enum Geometry {
    /// Plano infinito definido por ponto e normal
    Plane {
        point: [f64; 3],
        normal: [f64; 3],
    },
    /// Esfera definida por centro e raio
    Sphere {
        center: [f64; 3],
        radius: f64,
    },
    /// Triângulo isolado
    Triangle(Triangle),
    /// Conjunto de triângulos (malha)
    Mesh { triangles: Vec<Triangle> },
}

impl Geometry {
    fn intersect(&self, origin: [f64; 3], direction: [f64; 3]) -> Option<HitRecord> {
        match self {
            Geometry::Plane { point, normal } => {
                intersect_plane(*point, *normal, origin, direction)
            }
            Geometry::Sphere { center, radius } => {
                intersect_sphere(*center, *radius, origin, direction)
            }
            Geometry::Triangle(triangle) => triangle.intersect(origin, direction),
            Geometry::Mesh { triangles } => {
                let mut closest: Option<HitRecord> = None;
                for triangle in triangles {
                    if let Some(hit) = triangle.intersect(origin, direction) {
                        let update = closest
                            .map(|best| hit.distance < best.distance)
                            .unwrap_or(true);
                        if update {
                            closest = Some(hit);
                        }
                    }
                }
                closest
            }
        }
    }
}

fn intersect_plane(
    point: [f64; 3],
    normal: [f64; 3],
    origin: [f64; 3],
    direction: [f64; 3],
) -> Option<HitRecord> {
    let n = normalize(normal);
    let denom = dot(n, direction);
    if denom.abs() < EPSILON {
        return None;
    }

    let t = dot(sub(point, origin), n) / denom;
    if t <= EPSILON {
        return None;
    }

    let pos = add(origin, scale(direction, t));
    let oriented_normal = orient_normal(n, direction);

    Some(HitRecord {
        point: pos,
        normal: oriented_normal,
        distance: t,
    })
}

fn intersect_sphere(
    center: [f64; 3],
    radius: f64,
    origin: [f64; 3],
    direction: [f64; 3],
) -> Option<HitRecord> {
    let dir = normalize(direction);
    let oc = sub(origin, center);
    let radius = radius.abs();
    let a = dot(dir, dir);
    let b = 2.0 * dot(oc, dir);
    let c = dot(oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_disc = discriminant.sqrt();
    let mut t = (-b - sqrt_disc) / (2.0 * a);
    if t <= EPSILON {
        t = (-b + sqrt_disc) / (2.0 * a);
    }
    if t <= EPSILON {
        return None;
    }

    let point = add(origin, scale(dir, t));
    let mut normal = normalize(sub(point, center));
    normal = orient_normal(normal, dir);

    Some(HitRecord {
        point,
        normal,
        distance: t,
    })
}

/// Superfície na cena
#[derive(Debug, Clone)]
pub struct Surface {
    /// Geometria associada
    pub geometry: Geometry,
    /// Material da superfície
    pub material: Material,
}

impl Surface {
    /// Cria superfície plana
    pub fn plane(point: [f64; 3], normal: [f64; 3], material: Material) -> Self {
        Self {
            geometry: Geometry::Plane {
                point,
                normal: normalize(normal),
            },
            material,
        }
    }

    /// Cria superfície esférica
    pub fn sphere(center: [f64; 3], radius: f64, material: Material) -> Self {
        Self {
            geometry: Geometry::Sphere { center, radius },
            material,
        }
    }

    /// Cria superfície triangular
    pub fn triangle(v0: [f64; 3], v1: [f64; 3], v2: [f64; 3], material: Material) -> Self {
        Self {
            geometry: Geometry::Triangle(Triangle::new(v0, v1, v2)),
            material,
        }
    }

    /// Cria superfície a partir de triângulos arbitrários
    pub fn mesh(triangles: Vec<Triangle>, material: Material) -> Self {
        Self {
            geometry: Geometry::Mesh { triangles },
            material,
        }
    }

    /// Mantém compatibilidade com API anterior (plano)
    pub fn new(position: [f64; 3], normal: [f64; 3], material: Material) -> Self {
        Self::plane(position, normal, material)
    }

    /// Plano lambertiano
    pub fn lambertian(position: [f64; 3], normal: [f64; 3], albedo: f64) -> Self {
        Self::plane(position, normal, Material::lambertian(albedo))
    }

    /// Calcula interseção com raio
    pub fn intersect(&self, origin: [f64; 3], direction: [f64; 3]) -> Option<HitRecord> {
        self.geometry.intersect(origin, direction)
    }
}

/// Fonte de luz
#[derive(Debug, Clone)]
pub struct Light {
    /// Posição da luz
    pub position: [f64; 3],

    /// Intensidade (potência em Watts)
    pub intensity: f64,

    /// Cor (comprimento de onda em metros)
    pub wavelength: f64,

    /// Tipo de luz
    pub light_type: LightType,
}

/// Tipo de fonte de luz
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Luz pontual (onidirecional)
    Point,

    /// Luz direcional (sol)
    Directional,

    /// Luz spot (cone)
    Spot,

    /// Luz de área
    Area,
}

impl Light {
    /// Cria luz pontual
    pub fn point(position: [f64; 3], intensity: f64) -> Self {
        Self {
            position,
            intensity,
            wavelength: 550e-9, // Verde (550nm)
            light_type: LightType::Point,
        }
    }

    /// Cria luz direcional
    pub fn directional(direction: [f64; 3], intensity: f64) -> Self {
        Self {
            position: normalize_or(direction, [0.0, -1.0, 0.0]),
            intensity,
            wavelength: 550e-9,
            light_type: LightType::Directional,
        }
    }

    /// Energia do fóton (E = hc/λ)
    pub fn photon_energy(&self) -> f64 {
        use crate::{HBAR, SPEED_OF_LIGHT};
        2.0 * std::f64::consts::PI * HBAR * SPEED_OF_LIGHT / self.wavelength
    }

    /// Cria vértice de emissão a partir desta luz
    pub fn emit_vertex(&self, direction: [f64; 3]) -> Vertex {
        Vertex::emission(self.position, direction, self.photon_energy())
    }
}

/// Câmera/Sensor
#[derive(Debug, Clone)]
pub struct Camera {
    /// Posição da câmera
    pub position: [f64; 3],

    /// Direção de visão
    pub look_at: [f64; 3],

    /// Vetor "up"
    pub up: [f64; 3],

    /// Campo de visão (FOV em radianos)
    pub fov: f64,

    /// Resolução (largura, altura)
    pub resolution: (usize, usize),
}

impl Camera {
    /// Cria nova câmera
    pub fn new(position: [f64; 3], look_at: [f64; 3], fov: f64) -> Self {
        Self {
            position,
            look_at,
            up: [0.0, 1.0, 0.0],
            fov,
            resolution: (800, 600),
        }
    }

    /// Define resolução
    pub fn with_resolution(mut self, width: usize, height: usize) -> Self {
        self.resolution = (width, height);
        self
    }

    /// Gera raio para pixel (u, v) normalizado [0,1]
    pub fn generate_ray(&self, u: f64, v: f64) -> [f64; 3] {
        let aspect = self.resolution.0 as f64 / self.resolution.1 as f64;
        let theta = self.fov * 0.5;
        let h = theta.tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let x = (2.0 * u - 1.0) * viewport_width * 0.5;
        let y = (2.0 * v - 1.0) * viewport_height * 0.5;

        // Construir sistema ortonormal (camera basis)
        let forward = normalize(sub(self.look_at, self.position));
        let right = normalize(cross(forward, self.up));
        let up = normalize(cross(right, forward));

        let direction = add(forward, add(scale(right, x), scale(up, y)));
        normalize(direction)
    }

    /// Cria vértice de detecção para pixel
    pub fn detect_vertex(&self, u: f64, v: f64) -> Vertex {
        let _direction = self.generate_ray(u, v);
        Vertex::detection(self.position, 0.0)
    }
}

/// Cena completa
#[derive(Debug, Clone)]
pub struct Scene {
    /// Luzes na cena
    pub lights: Vec<Light>,

    /// Superfícies na cena
    pub surfaces: Vec<Surface>,

    /// Câmera
    pub camera: Camera,

    /// Meio ambiente (índice de refração)
    pub ambient_refractive_index: f64,
}

impl Scene {
    /// Cria cena vazia
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            surfaces: Vec::new(),
            camera: Camera::new(
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 0.0],
                std::f64::consts::FRAC_PI_3,
            ),
            ambient_refractive_index: 1.0,
        }
    }

    /// Adiciona luz à cena
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Adiciona superfície à cena
    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
    }

    /// Define câmera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Número total de objetos
    pub fn num_objects(&self) -> usize {
        self.lights.len() + self.surfaces.len()
    }

    /// Valida cena
    pub fn is_valid(&self) -> bool {
        !self.lights.is_empty() && !self.surfaces.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = Material::lambertian(0.8);
        assert_eq!(mat.reflectance(), 0.8);

        let glass = Material::glass();
        assert_eq!(glass.refractive_index(), 1.5);
    }

    #[test]
    fn test_plane_intersection() {
        let surface = Surface::plane([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], Material::mirror());
        let ray_origin = [0.0, -1.0, 0.0];
        let ray_dir = [0.0, 1.0, 0.0];

        let hit = surface.intersect(ray_origin, ray_dir).expect("should hit plane");
        assert!(hit.distance > 0.0);
        assert!((hit.point[1]).abs() < 1e-6);
        assert!(dot(hit.normal, ray_dir) <= 0.0);
    }

    #[test]
    fn test_sphere_intersection() {
        let surface = Surface::sphere([0.0, 0.0, 0.0], 1.0, Material::mirror());
        let hit = surface
            .intersect([0.0, 0.0, -3.0], [0.0, 0.0, 1.0])
            .expect("should hit sphere");

        assert!((hit.distance - 2.0).abs() < 1e-6);
        assert!(dot(hit.normal, [0.0, 0.0, 1.0]) <= 0.0);
    }

    #[test]
    fn test_triangle_intersection() {
        let surface = Surface::triangle(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            Material::lambertian(0.5),
        );

        let hit = surface.intersect([0.25, 0.25, 1.0], [0.0, 0.0, -1.0]);
        assert!(hit.is_some());
    }

    #[test]
    fn test_light_creation() {
        let light = Light::point([0.0, 5.0, 0.0], 100.0);
        assert!(light.photon_energy() > 0.0);
    }

    #[test]
    fn test_camera_ray_generation() {
        let camera = Camera::new(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 0.0],
            std::f64::consts::FRAC_PI_4,
        );
        let ray = camera.generate_ray(0.5, 0.5);
        assert!(ray[2] < 0.0);
    }

    #[test]
    fn test_scene_creation() {
        let mut scene = Scene::new();
        scene.add_light(Light::point([0.0, 5.0, 0.0], 100.0));
        scene.add_surface(Surface::lambertian(
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            0.8,
        ));

        assert!(scene.is_valid());
        assert_eq!(scene.num_objects(), 2);
    }
}//! Cena 3D para renderização quântica

use crate::photon::Vertex;

/// Material de superfície
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    /// Lambertiano (difuso ideal)
    Lambertian {
        /// Albedo (refletividade difusa) [0, 1]
        albedo: f64,
    },

    /// Especular (espelho)
    Specular {
        /// Refletância especular [0, 1]
        reflectance: f64,
    },

    /// Dielétrico (vidro, água)
    Dielectric {
        /// Índice de refração (n)
        refractive_index: f64,
        /// Transmitância [0, 1]
        transmittance: f64,
    },

    /// Metal (condutor)
    Metal {
        /// Refletância [0, 1]
        reflectance: f64,
        /// Rugosidade [0, 1]
        roughness: f64,
    },

    /// Absorvente (corpo negro)
    Absorbing {
        /// Coeficiente de absorção (m⁻¹)
        absorption_coeff: f64,
    },
}

impl Material {
    /// Cria material Lambertiano
    pub fn lambertian(albedo: f64) -> Self {
        Material::Lambertian {
            albedo: albedo.clamp(0.0, 1.0),
        }
    }

    /// Cria espelho perfeito
    pub fn mirror() -> Self {
        Material::Specular { reflectance: 1.0 }
    }

    /// Cria vidro (n ≈ 1.5)
    pub fn glass() -> Self {
        Material::Dielectric {
            refractive_index: 1.5,
            transmittance: 0.95,
        }
    }

    /// Cria metal
    pub fn metal(reflectance: f64) -> Self {
        Material::Metal {
            reflectance: reflectance.clamp(0.0, 1.0),
            roughness: 0.1,
        }
    }

    /// Retorna índice de refração (se aplicável)
    pub fn refractive_index(&self) -> f64 {
        match self {
            Material::Dielectric {
                refractive_index, ..
            } => *refractive_index,
            _ => 1.0,
        }
    }

    /// Retorna albedo/reflectância
    pub fn reflectance(&self) -> f64 {
        match self {
            Material::Lambertian { albedo } => *albedo,
            Material::Specular { reflectance } => *reflectance,
            Material::Metal { reflectance, .. } => *reflectance,
            Material::Dielectric { transmittance, .. } => 1.0 - transmittance,
            Material::Absorbing { .. } => 0.0,
        }
    }
}

/// Superfície na cena
#[derive(Debug, Clone)]
pub struct Surface {
    /// Posição da superfície
    pub position: [f64; 3],

    /// Normal da superfície
    pub normal: [f64; 3],

    /// Material
    pub material: Material,

    /// Área da superfície (para importance sampling)
    pub area: f64,
}

impl Surface {
    /// Cria nova superfície
    pub fn new(position: [f64; 3], normal: [f64; 3], material: Material) -> Self {
        Self {
            position,
            normal: Self::normalize(normal),
            material,
            area: 1.0,
        }
    }

    /// Cria superfície Lambertiana
    pub fn lambertian(position: [f64; 3], normal: [f64; 3], albedo: f64) -> Self {
        Self::new(position, normal, Material::lambertian(albedo))
    }

    /// Normaliza vetor
    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len > 1e-10 {
            [v[0] / len, v[1] / len, v[2] / len]
        } else {
            [0.0, 1.0, 0.0]
        }
    }

    /// Produto escalar com direção
    pub fn dot(&self, direction: [f64; 3]) -> f64 {
        self.normal[0] * direction[0]
            + self.normal[1] * direction[1]
            + self.normal[2] * direction[2]
    }

    /// Verifica se ponto está próximo da superfície
    pub fn is_near(&self, point: [f64; 3], tolerance: f64) -> bool {
        let dx = point[0] - self.position[0];
        let dy = point[1] - self.position[1];
        let dz = point[2] - self.position[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        dist_sq < tolerance * tolerance
    }
}

/// Fonte de luz
#[derive(Debug, Clone)]
pub struct Light {
    /// Posição da luz
    pub position: [f64; 3],

    /// Intensidade (potência em Watts)
    pub intensity: f64,

    /// Cor (comprimento de onda em metros)
    pub wavelength: f64,

    /// Tipo de luz
    pub light_type: LightType,
}

/// Tipo de fonte de luz
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Luz pontual (onidirecional)
    Point,

    /// Luz direcional (sol)
    Directional,

    /// Luz spot (cone)
    Spot,

    /// Luz de área
    Area,
}

impl Light {
    /// Cria luz pontual
    pub fn point(position: [f64; 3], intensity: f64) -> Self {
        Self {
            position,
            intensity,
            wavelength: 550e-9, // Verde (550nm)
            light_type: LightType::Point,
        }
    }

    /// Cria luz direcional
    pub fn directional(direction: [f64; 3], intensity: f64) -> Self {
        Self {
            position: direction,
            intensity,
            wavelength: 550e-9,
            light_type: LightType::Directional,
        }
    }

    /// Energia do fóton (E = hc/λ)
    pub fn photon_energy(&self) -> f64 {
        use crate::{HBAR, SPEED_OF_LIGHT};
        2.0 * std::f64::consts::PI * HBAR * SPEED_OF_LIGHT / self.wavelength
    }

    /// Cria vértice de emissão a partir desta luz
    pub fn emit_vertex(&self, direction: [f64; 3]) -> Vertex {
        Vertex::emission(self.position, direction, self.photon_energy())
    }
}

/// Câmera/Sensor
#[derive(Debug, Clone)]
pub struct Camera {
    /// Posição da câmera
    pub position: [f64; 3],

    /// Direção de visão
    pub look_at: [f64; 3],

    /// Vetor "up"
    pub up: [f64; 3],

    /// Campo de visão (FOV em radianos)
    pub fov: f64,

    /// Resolução (largura, altura)
    pub resolution: (usize, usize),
}

impl Camera {
    /// Cria nova câmera
    pub fn new(position: [f64; 3], look_at: [f64; 3], fov: f64) -> Self {
        Self {
            position,
            look_at,
            up: [0.0, 1.0, 0.0],
            fov,
            resolution: (800, 600),
        }
    }

    /// Define resolução
    pub fn with_resolution(mut self, width: usize, height: usize) -> Self {
        self.resolution = (width, height);
        self
    }

    /// Gera raio para pixel (u, v) normalizado [0,1]
    pub fn generate_ray(&self, u: f64, v: f64) -> [f64; 3] {
        // Calcula direção do raio baseado em FOV
        let aspect = self.resolution.0 as f64 / self.resolution.1 as f64;
        let theta = self.fov * 0.5;
        let h = theta.tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        // Coordenadas no plano de imagem
        let x = (2.0 * u - 1.0) * viewport_width * 0.5;
        let y = (2.0 * v - 1.0) * viewport_height * 0.5;

        // Direção normalizada
        let dir = [x, y, -1.0];
        Self::normalize(dir)
    }

    /// Cria vértice de detecção para pixel
    pub fn detect_vertex(&self, u: f64, v: f64) -> Vertex {
        let _direction = self.generate_ray(u, v);
        Vertex::detection(self.position, 0.0)
    }

    /// Normaliza vetor
    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if len > 1e-10 {
            [v[0] / len, v[1] / len, v[2] / len]
        } else {
            [0.0, 0.0, -1.0]
        }
    }
}

/// Cena completa
#[derive(Debug, Clone)]
pub struct Scene {
    /// Luzes na cena
    pub lights: Vec<Light>,

    /// Superfícies na cena
    pub surfaces: Vec<Surface>,

    /// Câmera
    pub camera: Camera,

    /// Meio ambiente (índice de refração)
    pub ambient_refractive_index: f64,
}

impl Scene {
    /// Cria cena vazia
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            surfaces: Vec::new(),
            camera: Camera::new(
                [0.0, 0.0, 5.0],
                [0.0, 0.0, 0.0],
                std::f64::consts::FRAC_PI_3,
            ),
            ambient_refractive_index: 1.0, // Vácuo/ar
        }
    }

    /// Adiciona luz à cena
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Adiciona superfície à cena
    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
    }

    /// Define câmera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    /// Número total de objetos
    pub fn num_objects(&self) -> usize {
        self.lights.len() + self.surfaces.len()
    }

    /// Valida cena
    pub fn is_valid(&self) -> bool {
        !self.lights.is_empty() && !self.surfaces.is_empty()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat = Material::lambertian(0.8);
        assert_eq!(mat.reflectance(), 0.8);

        let glass = Material::glass();
        assert_eq!(glass.refractive_index(), 1.5);
    }

    #[test]
    fn test_surface_creation() {
        let surf = Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8);
        assert_eq!(surf.material.reflectance(), 0.8);
    }

    #[test]
    fn test_light_creation() {
        let light = Light::point([0.0, 5.0, 0.0], 100.0);
        assert!(light.photon_energy() > 0.0);
    }

    #[test]
    fn test_camera_ray_generation() {
        let camera = Camera::new(
            [0.0, 0.0, 5.0],
            [0.0, 0.0, 0.0],
            std::f64::consts::FRAC_PI_4,
        );
        let ray = camera.generate_ray(0.5, 0.5);

        // Raio central deve apontar para frente
        assert!(ray[2] < 0.0);
    }

    #[test]
    fn test_scene_creation() {
        let mut scene = Scene::new();
        scene.add_light(Light::point([0.0, 5.0, 0.0], 100.0));
        scene.add_surface(Surface::lambertian([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.8));

        assert!(scene.is_valid());
        assert_eq!(scene.num_objects(), 2);
    }
}
