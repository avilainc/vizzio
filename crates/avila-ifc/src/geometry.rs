use crate::entities::IfcEntity;
use crate::error::{IfcError, Result};
use crate::file::IfcFile;
use glam::{DMat4, DVec3};

/// 3D point representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec(coords: &[f64]) -> Option<Self> {
        if coords.len() >= 3 {
            Some(Self::new(coords[0], coords[1], coords[2]))
        } else if coords.len() == 2 {
            Some(Self::new(coords[0], coords[1], 0.0))
        } else {
            None
        }
    }

    pub fn to_vec3(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }
}

impl From<DVec3> for Point3D {
    fn from(v: DVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

/// 3D direction vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Direction3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec(ratios: &[f64]) -> Option<Self> {
        if ratios.len() >= 3 {
            Some(Self::new(ratios[0], ratios[1], ratios[2]))
        } else if ratios.len() == 2 {
            Some(Self::new(ratios[0], ratios[1], 0.0))
        } else {
            None
        }
    }

    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            self
        }
    }

    pub fn to_vec3(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }
}

/// 3D transformation matrix
#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub matrix: DMat4,
}

impl Transform3D {
    pub fn identity() -> Self {
        Self {
            matrix: DMat4::IDENTITY,
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            matrix: DMat4::from_translation(DVec3::new(x, y, z)),
        }
    }

    pub fn from_axis2placement3d(
        location: Point3D,
        axis: Option<Direction3D>,
        ref_direction: Option<Direction3D>,
    ) -> Self {
        let z_axis = axis.unwrap_or(Direction3D::new(0.0, 0.0, 1.0)).normalize();
        let x_axis = ref_direction
            .unwrap_or(Direction3D::new(1.0, 0.0, 0.0))
            .normalize();

        // Compute y-axis as cross product
        let z_vec = z_axis.to_vec3();
        let x_vec = x_axis.to_vec3();
        let y_vec = z_vec.cross(x_vec).normalize();

        // Recompute x to ensure orthogonality
        let x_vec = y_vec.cross(z_vec).normalize();

        let matrix = DMat4::from_cols(
            x_vec.extend(0.0),
            y_vec.extend(0.0),
            z_vec.extend(0.0),
            location.to_vec3().extend(1.0),
        );

        Self { matrix }
    }

    pub fn transform_point(&self, point: Point3D) -> Point3D {
        let vec = self.matrix.transform_point3(point.to_vec3());
        Point3D::from(vec)
    }

    pub fn transform_direction(&self, direction: Direction3D) -> Direction3D {
        let vec = self.matrix.transform_vector3(direction.to_vec3());
        Direction3D::new(vec.x, vec.y, vec.z).normalize()
    }

    pub fn compose(&self, other: &Transform3D) -> Transform3D {
        Self {
            matrix: self.matrix * other.matrix,
        }
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::identity()
    }
}

/// Bounding box representation
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}

impl BoundingBox {
    pub fn new(min: Point3D, max: Point3D) -> Self {
        Self { min, max }
    }

    pub fn from_points(points: &[Point3D]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut min_x = points[0].x;
        let mut min_y = points[0].y;
        let mut min_z = points[0].z;
        let mut max_x = points[0].x;
        let mut max_y = points[0].y;
        let mut max_z = points[0].z;

        for point in points.iter().skip(1) {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }

        Some(Self {
            min: Point3D::new(min_x, min_y, min_z),
            max: Point3D::new(max_x, max_y, max_z),
        })
    }

    pub fn center(&self) -> Point3D {
        Point3D::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn dimensions(&self) -> (f64, f64, f64) {
        (
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn volume(&self) -> f64 {
        let (dx, dy, dz) = self.dimensions();
        dx * dy * dz
    }

    pub fn contains(&self, point: Point3D) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

/// Geometry processor for IFC entities
pub struct GeometryProcessor<'a> {
    file: &'a IfcFile,
}

impl<'a> GeometryProcessor<'a> {
    pub fn new(file: &'a IfcFile) -> Self {
        Self { file }
    }

    /// Extract cartesian point coordinates from entity
    pub fn get_cartesian_point(&self, entity: &IfcEntity) -> Result<Point3D> {
        if entity.entity_type != "IFCCARTESIANPOINT" {
            return Err(IfcError::GeometryError(
                "Entity is not an IfcCartesianPoint".to_string(),
            ));
        }

        let coords_list = entity.get_list_attribute(0)?.ok_or_else(|| {
            IfcError::GeometryError("Missing coordinates in cartesian point".to_string())
        })?;

        let coords: Vec<f64> = coords_list
            .iter()
            .filter_map(|v| match v {
                crate::step_parser::StepValue::Real(r) => Some(*r),
                crate::step_parser::StepValue::Integer(i) => Some(*i as f64),
                _ => None,
            })
            .collect();

        Point3D::from_vec(&coords).ok_or_else(|| {
            IfcError::GeometryError("Invalid coordinates in cartesian point".to_string())
        })
    }

    /// Extract direction ratios from entity
    pub fn get_direction(&self, entity: &IfcEntity) -> Result<Direction3D> {
        if entity.entity_type != "IFCDIRECTION" {
            return Err(IfcError::GeometryError(
                "Entity is not an IfcDirection".to_string(),
            ));
        }

        let ratios_list = entity.get_list_attribute(0)?.ok_or_else(|| {
            IfcError::GeometryError("Missing direction ratios".to_string())
        })?;

        let ratios: Vec<f64> = ratios_list
            .iter()
            .filter_map(|v| match v {
                crate::step_parser::StepValue::Real(r) => Some(*r),
                crate::step_parser::StepValue::Integer(i) => Some(*i as f64),
                _ => None,
            })
            .collect();

        Direction3D::from_vec(&ratios).ok_or_else(|| {
            IfcError::GeometryError("Invalid direction ratios".to_string())
        })
    }

    /// Extract transformation from IfcAxis2Placement3D
    pub fn get_axis2placement3d(&self, entity: &IfcEntity) -> Result<Transform3D> {
        if entity.entity_type != "IFCAXIS2PLACEMENT3D" {
            return Err(IfcError::GeometryError(
                "Entity is not an IfcAxis2Placement3D".to_string(),
            ));
        }

        // Get location (required)
        let location_ref = entity.get_entity_ref_attribute(0)?.ok_or_else(|| {
            IfcError::GeometryError("Missing location in axis2placement3d".to_string())
        })?;
        let location_entity = self.file.resolve_reference(location_ref)?;
        let location = self.get_cartesian_point(&location_entity)?;

        // Get axis (optional)
        let axis = entity
            .get_entity_ref_attribute(1)?
            .and_then(|ref_id| self.file.get_entity(ref_id))
            .and_then(|e| self.get_direction(&e).ok());

        // Get ref_direction (optional)
        let ref_direction = entity
            .get_entity_ref_attribute(2)?
            .and_then(|ref_id| self.file.get_entity(ref_id))
            .and_then(|e| self.get_direction(&e).ok());

        Ok(Transform3D::from_axis2placement3d(
            location,
            axis,
            ref_direction,
        ))
    }

    /// Get local placement transformation
    pub fn get_local_placement(&self, entity_id: i64) -> Result<Transform3D> {
        let entity = self.file.resolve_reference(entity_id)?;

        if entity.entity_type != "IFCLOCALPLACEMENT" {
            return Err(IfcError::GeometryError(
                "Entity is not an IfcLocalPlacement".to_string(),
            ));
        }

        // Get relative placement
        let relative_placement_ref = entity.get_entity_ref_attribute(1)?.ok_or_else(|| {
            IfcError::GeometryError("Missing relative placement".to_string())
        })?;
        let placement_entity = self.file.resolve_reference(relative_placement_ref)?;
        let local_transform = self.get_axis2placement3d(&placement_entity)?;

        // Get parent placement (optional)
        if let Some(parent_ref) = entity.get_entity_ref_attribute(0)? {
            let parent_transform = self.get_local_placement(parent_ref)?;
            Ok(parent_transform.compose(&local_transform))
        } else {
            Ok(local_transform)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_creation() {
        let point = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_direction_normalize() {
        let dir = Direction3D::new(3.0, 4.0, 0.0).normalize();
        let len = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
        assert!((len - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(10.0, 10.0, 10.0),
            Point3D::new(5.0, 5.0, 5.0),
        ];
        let bbox = BoundingBox::from_points(&points).unwrap();
        assert_eq!(bbox.min, Point3D::new(0.0, 0.0, 0.0));
        assert_eq!(bbox.max, Point3D::new(10.0, 10.0, 10.0));
        assert_eq!(bbox.volume(), 1000.0);
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(10.0, 10.0, 10.0),
        );
        assert!(bbox.contains(Point3D::new(5.0, 5.0, 5.0)));
        assert!(!bbox.contains(Point3D::new(15.0, 5.0, 5.0)));
    }

    #[test]
    fn test_transform_identity() {
        let transform = Transform3D::identity();
        let point = Point3D::new(1.0, 2.0, 3.0);
        let transformed = transform.transform_point(point);
        assert_eq!(transformed, point);
    }
}
