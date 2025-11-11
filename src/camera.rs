// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! 3D camera with view and projection matrices.
//!
//! This module provides a [`Camera`] struct implementing a target-based camera system
//! with perspective projection. The camera supports:
//! - WASD movement (forward, back, left, right, up, down)
//! - Mouse-based rotation (horizontal yaw and vertical pitch)
//! - Configurable field of view and clipping planes
//!
//! # Example
//!
//! ```rust,no_run
//! use projectrigor::camera::Camera;
//!
//! let mut camera = Camera::new(16.0 / 9.0); // 16:9 aspect ratio
//!
//! // Get matrices for rendering
//! let view = camera.view_matrix();
//! let projection = camera.projection_matrix();
//! let view_projection = camera.view_projection_matrix();
//!
//! // Move camera
//! camera.move_forward(0.1);
//! camera.rotate_horizontal(0.05); // Rotate right
//! ```

use glam::{Mat4, Vec3};

/// A perspective camera with position and target.
///
/// The camera uses a "look-at" system where it's positioned at `position` and
/// always faces towards `target`. It supports free movement in 3D space and
/// rotation around the target point.
///
/// # Fields
///
/// - `position`: Camera's world position
/// - `target`: Point the camera looks at
/// - `up`: World up vector (typically (0, 1, 0))
/// - `fov`: Vertical field of view in radians
/// - `aspect_ratio`: Width / height of viewport
/// - `near`: Near clipping plane distance
/// - `far`: Far clipping plane distance
///
/// # Example
///
/// ```rust
/// use projectrigor::camera::Camera;
/// use glam::Vec3;
///
/// let mut camera = Camera::new(1920.0 / 1080.0);
/// camera.set_position(Vec3::new(0.0, 5.0, 10.0));
/// camera.set_target(Vec3::ZERO);
/// ```
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
    /// Creates a new camera with default position and orientation.
    ///
    /// Default configuration:
    /// - Position: (8, 4, 12) - elevated and pulled back
    /// - Target: (6, 0, 0) - looking at center of scene
    /// - Up: (0, 1, 0) - Y-up coordinate system
    /// - FOV: 45° vertical
    /// - Near plane: 0.1 units
    /// - Far plane: 100 units
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - Width / height of the viewport (e.g., 16.0/9.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// ```
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

    /// Returns the view matrix for rendering.
    ///
    /// The view matrix transforms world coordinates to camera/view space.
    /// Uses right-handed coordinate system.
    ///
    /// # Returns
    ///
    /// A 4×4 view matrix
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let view = camera.view_matrix();
    /// ```
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Returns the projection matrix for rendering.
    ///
    /// The projection matrix transforms view space to clip space using
    /// perspective projection. Uses right-handed coordinate system.
    ///
    /// # Returns
    ///
    /// A 4×4 perspective projection matrix
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let projection = camera.projection_matrix();
    /// ```
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    /// Returns the combined view-projection matrix.
    ///
    /// This is equivalent to `projection_matrix() * view_matrix()` and
    /// transforms world coordinates directly to clip space.
    ///
    /// # Returns
    ///
    /// A 4×4 view-projection matrix
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let vp = camera.view_projection_matrix();
    /// // Use vp matrix in shader
    /// ```
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Updates the camera's aspect ratio.
    ///
    /// Call this when the window is resized to maintain correct perspective.
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - New width / height ratio
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.update_aspect_ratio(21.0 / 9.0); // Ultrawide
    /// ```
    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Sets the camera's world position.
    ///
    /// # Arguments
    ///
    /// * `position` - New camera position
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    /// use glam::Vec3;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.set_position(Vec3::new(0.0, 10.0, 10.0));
    /// ```
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Sets the camera's target point (where it looks).
    ///
    /// # Arguments
    ///
    /// * `target` - New target position
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    /// use glam::Vec3;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.set_target(Vec3::ZERO); // Look at origin
    /// ```
    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
    }

    /// Returns the normalized forward direction vector.
    ///
    /// This is the direction from camera position to target.
    ///
    /// # Returns
    ///
    /// Normalized vector pointing from camera towards target
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let forward = camera.forward();
    /// ```
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    /// Returns the normalized right direction vector.
    ///
    /// This is perpendicular to both forward and up vectors.
    ///
    /// # Returns
    ///
    /// Normalized vector pointing to the camera's right
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let right = camera.right();
    /// ```
    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    /// Moves the camera forward along its viewing direction.
    ///
    /// Both position and target move together, maintaining the same viewing angle.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move (positive = forward, negative = backward)
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_forward(1.0); // Move 1 unit forward
    /// ```
    pub fn move_forward(&mut self, distance: f32) {
        let forward = self.forward();
        self.position += forward * distance;
        self.target += forward * distance;
    }

    /// Moves the camera backward along its viewing direction.
    ///
    /// Both position and target move together, maintaining the same viewing angle.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move backward
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_backward(1.0); // Move 1 unit backward
    /// ```
    pub fn move_backward(&mut self, distance: f32) {
        let forward = self.forward();
        self.position -= forward * distance;
        self.target -= forward * distance;
    }

    /// Moves the camera left perpendicular to its viewing direction.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move left
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_left(0.5); // Strafe left
    /// ```
    pub fn move_left(&mut self, distance: f32) {
        let right = self.right();
        self.position -= right * distance;
        self.target -= right * distance;
    }

    /// Moves the camera right perpendicular to its viewing direction.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move right
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_right(0.5); // Strafe right
    /// ```
    pub fn move_right(&mut self, distance: f32) {
        let right = self.right();
        self.position += right * distance;
        self.target += right * distance;
    }

    /// Moves the camera up along the world up axis.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move up
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_up(1.0); // Ascend
    /// ```
    pub fn move_up(&mut self, distance: f32) {
        self.position += self.up * distance;
        self.target += self.up * distance;
    }

    /// Moves the camera down along the world up axis.
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to move down
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.move_down(1.0); // Descend
    /// ```
    pub fn move_down(&mut self, distance: f32) {
        self.position -= self.up * distance;
        self.target -= self.up * distance;
    }

    /// Returns the camera's current world position.
    ///
    /// # Returns
    ///
    /// The camera's position vector
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let pos = camera.position();
    /// println!("Camera at: {:?}", pos);
    /// ```
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the camera's current target position.
    ///
    /// # Returns
    ///
    /// The point the camera is looking at
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    ///
    /// let camera = Camera::new(16.0 / 9.0);
    /// let target = camera.target();
    /// println!("Looking at: {:?}", target);
    /// ```
    pub fn target(&self) -> Vec3 {
        self.target
    }

    /// Rotates the camera horizontally (yaw) around the world up axis.
    ///
    /// Positive angles rotate to the right, negative to the left. The camera
    /// orbits around the target point while maintaining its distance.
    ///
    /// # Arguments
    ///
    /// * `angle_radians` - Rotation angle in radians (positive = right, negative = left)
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    /// use std::f32::consts::PI;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.rotate_horizontal(PI / 4.0); // Rotate 45° to the right
    /// ```
    pub fn rotate_horizontal(&mut self, angle_radians: f32) {
        let offset = self.position - self.target;
        let distance = offset.length();
        
        // Rotate around Y axis (world up)
        let cos_angle = angle_radians.cos();
        let sin_angle = angle_radians.sin();
        
        let new_x = offset.x * cos_angle - offset.z * sin_angle;
        let new_z = offset.x * sin_angle + offset.z * cos_angle;
        
        let new_offset = Vec3::new(new_x, offset.y, new_z).normalize() * distance;
        self.position = self.target + new_offset;
    }

    /// Rotates the camera vertically (pitch) around the right axis.
    ///
    /// Positive angles rotate upward, negative downward. The camera orbits
    /// around the target point while maintaining its distance. Rotation is
    /// clamped to prevent gimbal lock (nearly ±90° but not exactly).
    ///
    /// # Arguments
    ///
    /// * `angle_radians` - Rotation angle in radians (positive = up, negative = down)
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::camera::Camera;
    /// use std::f32::consts::PI;
    ///
    /// let mut camera = Camera::new(16.0 / 9.0);
    /// camera.rotate_vertical(PI / 6.0); // Tilt up 30°
    /// ```
    pub fn rotate_vertical(&mut self, angle_radians: f32) {
        let offset = self.position - self.target;
        let distance = offset.length();
        
        // Calculate current pitch angle
        let xz_distance = (offset.x * offset.x + offset.z * offset.z).sqrt();
        let current_pitch = offset.y.atan2(xz_distance);
        
        // Clamp to prevent flipping (allow nearly straight up/down but not exactly)
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        let min_pitch = -std::f32::consts::FRAC_PI_2 + 0.01;
        let new_pitch = (current_pitch + angle_radians).clamp(min_pitch, max_pitch);
        
        // Calculate new position maintaining distance
        let new_y = distance * new_pitch.sin();
        let new_xz_distance = distance * new_pitch.cos();
        
        // Maintain the same horizontal direction
        let horizontal_dir = Vec3::new(offset.x, 0.0, offset.z).normalize();
        let new_offset = Vec3::new(
            horizontal_dir.x * new_xz_distance,
            new_y,
            horizontal_dir.z * new_xz_distance,
        );
        
        self.position = self.target + new_offset;
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

    #[test]
    fn test_forward_direction() {
        let camera = Camera::new(16.0 / 9.0);
        let forward = camera.forward();
        // Forward should be normalized
        assert!((forward.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_right_direction() {
        let camera = Camera::new(16.0 / 9.0);
        let right = camera.right();
        let forward = camera.forward();
        // Right should be perpendicular to forward
        assert!(right.dot(forward).abs() < 0.001);
        // Right should be normalized
        assert!((right.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_move_forward() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        let initial_target = camera.target();
        
        camera.move_forward(1.0);
        
        let new_pos = camera.position();
        let new_target = camera.target();
        
        // Both position and target should have moved
        assert_ne!(initial_pos, new_pos);
        assert_ne!(initial_target, new_target);
        
        // Direction should remain the same
        let initial_dir = (initial_target - initial_pos).normalize();
        let new_dir = (new_target - new_pos).normalize();
        assert!((initial_dir - new_dir).length() < 0.001);
    }

    #[test]
    fn test_move_backward() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        
        camera.move_backward(1.0);
        
        let new_pos = camera.position();
        assert_ne!(initial_pos, new_pos);
    }

    #[test]
    fn test_move_left_right() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        
        camera.move_right(1.0);
        let after_right = camera.position();
        assert_ne!(initial_pos, after_right);
        
        camera.move_left(2.0);
        let after_left = camera.position();
        
        // Moving right then left by 2x should put us to the left of start
        let right_movement = after_right - initial_pos;
        let total_movement = after_left - initial_pos;
        // Should have moved in opposite direction
        assert!(right_movement.dot(total_movement) < 0.0);
    }

    #[test]
    fn test_move_up_down() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        
        camera.move_up(1.0);
        let new_pos = camera.position();
        
        // Y component should have increased
        assert!(new_pos.y > initial_pos.y);
        
        camera.move_down(2.0);
        let final_pos = camera.position();
        
        // Y component should now be less than initial
        assert!(final_pos.y < initial_pos.y);
    }

    #[test]
    fn test_rotate_horizontal() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        let target = camera.target();
        
        // Rotate 90 degrees (PI/2 radians) to the right
        camera.rotate_horizontal(std::f32::consts::FRAC_PI_2);
        
        let new_pos = camera.position();
        
        // Position should have changed
        assert_ne!(initial_pos, new_pos);
        
        // Target should remain the same
        assert_eq!(target, camera.target());
        
        // Distance from target should be preserved
        let initial_dist = (initial_pos - target).length();
        let new_dist = (new_pos - target).length();
        assert!((initial_dist - new_dist).abs() < 0.001);
    }

    #[test]
    fn test_rotate_vertical() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        let target = camera.target();
        
        // Rotate 45 degrees up
        camera.rotate_vertical(std::f32::consts::FRAC_PI_4);
        
        let new_pos = camera.position();
        
        // Position should have changed
        assert_ne!(initial_pos, new_pos);
        
        // Y component should have increased (rotated up)
        assert!(new_pos.y > initial_pos.y);
        
        // Target should remain the same
        assert_eq!(target, camera.target());
        
        // Distance from target should be preserved
        let initial_dist = (initial_pos - target).length();
        let new_dist = (new_pos - target).length();
        assert!((initial_dist - new_dist).abs() < 0.001);
    }

    #[test]
    fn test_rotate_vertical_clamping() {
        let mut camera = Camera::new(16.0 / 9.0);
        let target = camera.target();
        
        // Try to rotate way beyond vertical (should be clamped)
        camera.rotate_vertical(std::f32::consts::PI); // 180 degrees
        
        let pos = camera.position();
        
        // Should not flip over completely
        // Y should be above target but not flipped
        let offset = pos - target;
        assert!(offset.y > 0.0); // Still above target, not flipped
    }

    #[test]
    fn test_rotation_preserves_distance() {
        let mut camera = Camera::new(16.0 / 9.0);
        let target = camera.target();
        let initial_dist = (camera.position() - target).length();
        
        // Apply multiple rotations
        camera.rotate_horizontal(0.5);
        camera.rotate_vertical(0.3);
        camera.rotate_horizontal(-0.2);
        camera.rotate_vertical(-0.1);
        
        let final_dist = (camera.position() - target).length();
        
        // Distance should be preserved throughout rotations
        assert!((initial_dist - final_dist).abs() < 0.001);
    }

    #[test]
    fn test_full_rotation_horizontal() {
        let mut camera = Camera::new(16.0 / 9.0);
        let initial_pos = camera.position();
        
        // Rotate full circle (2*PI)
        camera.rotate_horizontal(std::f32::consts::TAU);
        
        let final_pos = camera.position();
        
        // Should be back at approximately the same position
        assert!((initial_pos - final_pos).length() < 0.01);
    }
}
