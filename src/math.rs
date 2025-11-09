// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Mat4, Quat, Vec3};

/// Represents a 3D transform with position, rotation, and scale
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Create a new transform at the origin with no rotation and unit scale
    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Create a transform at a specific position
    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Create a transform with position and rotation
    pub fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self {
            position,
            rotation,
            scale: Vec3::ONE,
        }
    }

    /// Convert to a 4x4 transformation matrix
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Transform a point from local to world space
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (point * self.scale) + self.position
    }

    /// Transform a vector (direction) from local to world space
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (vector * self.scale)
    }

    /// Rotate the transform by a quaternion
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    /// Translate the transform by a vector
    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_identity_transform() {
        let transform = Transform::identity();
        assert_eq!(transform.position, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    #[test]
    fn test_transform_point() {
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let point = Vec3::new(1.0, 0.0, 0.0);
        let transformed = transform.transform_point(point);
        assert_eq!(transformed, Vec3::new(2.0, 2.0, 3.0));
    }

    #[test]
    fn test_transform_rotation() {
        let rotation = Quat::from_rotation_y(PI / 2.0); // 90 degrees around Y
        let transform = Transform::from_position_rotation(Vec3::ZERO, rotation);
        let point = Vec3::new(1.0, 0.0, 0.0);
        let transformed = transform.transform_point(point);
        
        // After 90° rotation around Y, X becomes -Z
        assert!((transformed.x - 0.0).abs() < 0.001);
        assert!((transformed.y - 0.0).abs() < 0.001);
        assert!((transformed.z - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_to_matrix() {
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let matrix = transform.to_matrix();
        let point = Vec3::new(1.0, 0.0, 0.0);
        let transformed = matrix.transform_point3(point);
        assert_eq!(transformed, Vec3::new(2.0, 2.0, 3.0));
    }

    #[test]
    fn test_translate() {
        let mut transform = Transform::identity();
        transform.translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(transform.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_rotate() {
        let mut transform = Transform::identity();
        let rotation = Quat::from_rotation_y(PI / 2.0);
        transform.rotate(rotation);
        
        let point = Vec3::new(1.0, 0.0, 0.0);
        let transformed = transform.transform_point(point);
        assert!((transformed.z - (-1.0)).abs() < 0.001);
    }
}
