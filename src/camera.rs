// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use glam::{Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            position: Vec3::new(8.0, 4.0, 12.0), // Pull back and raise camera to see all boxes
            target: Vec3::new(6.0, 0.0, 0.0), // Look at center of spread
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 45.0_f32.to_radians(),
            aspect_ratio,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(16.0 / 9.0);
        assert_eq!(camera.position, Vec3::new(8.0, 4.0, 12.0));
        assert_eq!(camera.target, Vec3::new(6.0, 0.0, 0.0));
    }

    #[test]
    fn test_view_matrix() {
        let camera = Camera::new(16.0 / 9.0);
        let view = camera.view_matrix();
        // View matrix should be non-identity
        assert_ne!(view, Mat4::IDENTITY);
    }

    #[test]
    fn test_projection_matrix() {
        let camera = Camera::new(16.0 / 9.0);
        let proj = camera.projection_matrix();
        // Projection matrix should be non-identity
        assert_ne!(proj, Mat4::IDENTITY);
    }

    #[test]
    fn test_aspect_ratio_update() {
        let mut camera = Camera::new(16.0 / 9.0);
        camera.update_aspect_ratio(4.0 / 3.0);
        assert_eq!(camera.aspect_ratio, 4.0 / 3.0);
    }
}
