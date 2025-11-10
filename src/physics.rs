// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Physics simulation using the Rapier physics engine.
//!
//! This module provides a wrapper around the Rapier3D physics engine, offering
//! a simplified interface for creating rigid bodies, adding colliders, and
//! stepping the simulation.
//!
//! # Features
//!
//! - Dynamic and static rigid bodies
//! - Box and sphere colliders
//! - Gravity simulation (default: -9.81 m/s² on Y axis)
//! - Transform synchronization with ECS
//! - Velocity control
//!
//! # Example
//!
//! ```rust
//! use project_rigor::physics::PhysicsWorld;
//! use project_rigor::math::Transform;
//! use glam::Vec3;
//!
//! let mut physics = PhysicsWorld::new();
//!
//! // Create a dynamic cube
//! let transform = Transform::from_position(Vec3::new(0.0, 5.0, 0.0));
//! let body = physics.create_dynamic_body(transform);
//! physics.add_box_collider(body, Vec3::new(0.5, 0.5, 0.5));
//!
//! // Simulate physics
//! for _ in 0..60 {
//!     physics.step();
//! }
//!
//! // Get updated position
//! if let Some(new_transform) = physics.get_transform(body) {
//!     println!("Object fell to: {:?}", new_transform.position);
//! }
//! ```

use crate::math::Transform;
use glam::{Quat, Vec3};
use rapier3d::prelude::*;
use rapier3d::na::{Quaternion, UnitQuaternion};

/// A physics simulation world managing rigid bodies and colliders.
///
/// The `PhysicsWorld` wraps Rapier3D's physics engine and provides a simplified
/// interface for common physics operations. It handles:
/// - Rigid body creation (dynamic and static)
/// - Collider attachment (boxes and spheres)
/// - Physics simulation stepping
/// - Transform queries and updates
///
/// # Example
///
/// ```rust
/// use project_rigor::physics::PhysicsWorld;
/// use project_rigor::math::Transform;
/// use glam::Vec3;
///
/// let mut physics = PhysicsWorld::new();
///
/// // Create ground
/// let ground_transform = Transform::from_position(Vec3::new(0.0, -1.0, 0.0));
/// let ground = physics.create_static_body(ground_transform);
/// physics.add_box_collider(ground, Vec3::new(10.0, 0.1, 10.0));
/// ```
pub struct PhysicsWorld {
    /// Gravity vector (default: -9.81 on Y axis)
    pub gravity: Vec3,
    /// Physics integration parameters (timestep, iterations, etc.)
    pub integration_parameters: IntegrationParameters,
    /// The main physics pipeline
    pub physics_pipeline: PhysicsPipeline,
    /// Manages simulation islands for optimization
    pub island_manager: IslandManager,
    /// Broad-phase collision detection
    pub broad_phase: DefaultBroadPhase,
    /// Narrow-phase collision detection
    pub narrow_phase: NarrowPhase,
    /// Set of all rigid bodies
    pub rigid_body_set: RigidBodySet,
    /// Set of all colliders
    pub collider_set: ColliderSet,
    /// Impulse-based joints
    pub impulse_joint_set: ImpulseJointSet,
    /// Articulation/multibody joints
    pub multibody_joint_set: MultibodyJointSet,
    /// Continuous collision detection solver
    pub ccd_solver: CCDSolver,
    /// Query pipeline for raycasts and spatial queries
    pub query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    /// Creates a new physics world with default gravity (-9.81 on Y axis).
    ///
    /// This is equivalent to calling `PhysicsWorld::with_gravity(Vec3::new(0.0, -9.81, 0.0))`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    ///
    /// let physics = PhysicsWorld::new();
    /// ```
    pub fn new() -> Self {
        Self::with_gravity(Vec3::new(0.0, -9.81, 0.0))
    }

    /// Creates a physics world with custom gravity.
    ///
    /// # Arguments
    ///
    /// * `gravity` - The gravity vector in m/s² (e.g., Vec3::new(0.0, -9.81, 0.0) for Earth gravity)
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use glam::Vec3;
    ///
    /// // Moon gravity (approximately 1/6 of Earth)
    /// let physics = PhysicsWorld::with_gravity(Vec3::new(0.0, -1.62, 0.0));
    /// ```
    pub fn with_gravity(gravity: Vec3) -> Self {
        Self {
            gravity,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    /// Advances the physics simulation by one timestep.
    ///
    /// This updates all rigid body positions, velocities, and rotations based on
    /// forces, gravity, and collisions. The default timestep is 1/60th of a second.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// physics.step(); // Advance by ~16.67ms
    /// ```
    pub fn step(&mut self) {
        let gravity_vector = vector![self.gravity.x, self.gravity.y, self.gravity.z];

        self.physics_pipeline.step(
            &gravity_vector,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    /// Creates a dynamic rigid body at the specified transform.
    ///
    /// Dynamic bodies are affected by forces, gravity, and collisions. They're used
    /// for objects that should move and respond to physics (falling boxes, projectiles, etc.).
    ///
    /// # Arguments
    ///
    /// * `transform` - Initial position, rotation, and scale
    ///
    /// # Returns
    ///
    /// A handle to the created rigid body. Use this handle to add colliders or query the body.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let transform = Transform::from_position(Vec3::new(0.0, 10.0, 0.0));
    /// let body = physics.create_dynamic_body(transform);
    /// ```
    pub fn create_dynamic_body(&mut self, transform: Transform) -> RigidBodyHandle {
        let rotation = UnitQuaternion::from_quaternion(Quaternion::new(
            transform.rotation.w,
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
        ));

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![
                transform.position.x,
                transform.position.y,
                transform.position.z
            ])
            .rotation(rotation.scaled_axis())
            .build();

        self.rigid_body_set.insert(rigid_body)
    }

    /// Creates a static rigid body at the specified transform.
    ///
    /// Static bodies are immovable and unaffected by forces or gravity. They're used
    /// for ground planes, walls, and other fixed geometry that dynamic objects can
    /// collide with but cannot move.
    ///
    /// # Arguments
    ///
    /// * `transform` - Initial position, rotation, and scale
    ///
    /// # Returns
    ///
    /// A handle to the created rigid body.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let transform = Transform::from_position(Vec3::new(0.0, -1.0, 0.0));
    /// let ground = physics.create_static_body(transform);
    /// physics.add_box_collider(ground, Vec3::new(10.0, 0.1, 10.0));
    /// ```
    pub fn create_static_body(&mut self, transform: Transform) -> RigidBodyHandle {
        let rotation = UnitQuaternion::from_quaternion(Quaternion::new(
            transform.rotation.w,
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
        ));

        let rigid_body = RigidBodyBuilder::fixed()
            .translation(vector![
                transform.position.x,
                transform.position.y,
                transform.position.z
            ])
            .rotation(rotation.scaled_axis())
            .build();

        self.rigid_body_set.insert(rigid_body)
    }

    /// Adds a box-shaped collider to a rigid body.
    ///
    /// The box collider defines the physical shape used for collision detection.
    /// The size is specified as half-extents (distance from center to each face).
    ///
    /// # Arguments
    ///
    /// * `body_handle` - The rigid body to attach the collider to
    /// * `half_extents` - Half-width, half-height, half-depth (e.g., Vec3::new(0.5, 0.5, 0.5) for a 1×1×1 box)
    ///
    /// # Returns
    ///
    /// A handle to the created collider.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let body = physics.create_dynamic_body(Transform::default());
    /// // Create a 2×2×2 box collider
    /// physics.add_box_collider(body, Vec3::new(1.0, 1.0, 1.0));
    /// ```
    pub fn add_box_collider(
        &mut self,
        body_handle: RigidBodyHandle,
        half_extents: Vec3,
    ) -> ColliderHandle {
        let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y, half_extents.z)
            .build();

        self.collider_set
            .insert_with_parent(collider, body_handle, &mut self.rigid_body_set)
    }

    /// Adds a sphere-shaped collider to a rigid body.
    ///
    /// The sphere collider defines a spherical physical shape used for collision detection.
    ///
    /// # Arguments
    ///
    /// * `body_handle` - The rigid body to attach the collider to
    /// * `radius` - The radius of the sphere
    ///
    /// # Returns
    ///
    /// A handle to the created collider.
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let body = physics.create_dynamic_body(Transform::from_position(Vec3::new(0.0, 5.0, 0.0)));
    /// physics.add_sphere_collider(body, 0.5); // 0.5 unit radius sphere
    /// ```
    pub fn add_sphere_collider(
        &mut self,
        body_handle: RigidBodyHandle,
        radius: f32,
    ) -> ColliderHandle {
        let collider = ColliderBuilder::ball(radius).build();

        self.collider_set
            .insert_with_parent(collider, body_handle, &mut self.rigid_body_set)
    }

    /// Retrieves the current transform of a rigid body.
    ///
    /// This returns the rigid body's current position and rotation in world space.
    /// The scale component is always set to (1, 1, 1) as physics bodies don't have scale.
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle of the rigid body to query
    ///
    /// # Returns
    ///
    /// - `Some(Transform)`: The current transform if the body exists
    /// - `None`: If the handle is invalid or the body was removed
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let body = physics.create_dynamic_body(Transform::from_position(Vec3::new(0.0, 5.0, 0.0)));
    ///
    /// if let Some(transform) = physics.get_transform(body) {
    ///     println!("Body position: {:?}", transform.position);
    /// }
    /// ```
    pub fn get_transform(&self, handle: RigidBodyHandle) -> Option<Transform> {
        self.rigid_body_set.get(handle).map(|body| {
            let translation = body.translation();
            let rotation = body.rotation();

            Transform {
                position: Vec3::new(translation.x, translation.y, translation.z),
                rotation: Quat::from_xyzw(rotation.i, rotation.j, rotation.k, rotation.w),
                scale: Vec3::ONE,
            }
        })
    }

    /// Sets the linear velocity of a rigid body.
    ///
    /// This immediately changes the body's velocity, overriding any existing velocity.
    /// The body will continue moving at this velocity until affected by forces, gravity,
    /// or collisions.
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle of the rigid body to modify
    /// * `velocity` - The new linear velocity in m/s
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let body = physics.create_dynamic_body(Transform::default());
    ///
    /// // Launch the body upward at 10 m/s
    /// physics.set_velocity(body, Vec3::new(0.0, 10.0, 0.0));
    /// ```
    pub fn set_velocity(&mut self, handle: RigidBodyHandle, velocity: Vec3) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.set_linvel(vector![velocity.x, velocity.y, velocity.z], true);
        }
    }

    /// Gets the current linear velocity of a rigid body.
    ///
    /// # Arguments
    ///
    /// * `handle` - The handle of the rigid body to query
    ///
    /// # Returns
    ///
    /// - `Some(Vec3)`: The current linear velocity in m/s
    /// - `None`: If the handle is invalid or the body was removed
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::physics::PhysicsWorld;
    /// use project_rigor::math::Transform;
    /// use glam::Vec3;
    ///
    /// let mut physics = PhysicsWorld::new();
    /// let body = physics.create_dynamic_body(Transform::from_position(Vec3::new(0.0, 5.0, 0.0)));
    ///
    /// physics.step();
    /// if let Some(velocity) = physics.get_velocity(body) {
    ///     println!("Body velocity: {:?}", velocity);
    /// }
    /// ```
    pub fn get_velocity(&self, handle: RigidBodyHandle) -> Option<Vec3> {
        self.rigid_body_set.get(handle).map(|body| {
            let vel = body.linvel();
            Vec3::new(vel.x, vel.y, vel.z)
        })
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_physics_world() {
        let world = PhysicsWorld::new();
        assert_eq!(world.gravity, Vec3::new(0.0, -9.81, 0.0));
        assert_eq!(world.rigid_body_set.len(), 0);
    }

    #[test]
    fn test_create_dynamic_body() {
        let mut world = PhysicsWorld::new();
        let transform = Transform::from_position(Vec3::new(0.0, 10.0, 0.0));
        let handle = world.create_dynamic_body(transform);

        assert_eq!(world.rigid_body_set.len(), 1);

        let body_transform = world.get_transform(handle).unwrap();
        assert_eq!(body_transform.position, Vec3::new(0.0, 10.0, 0.0));
    }

    #[test]
    fn test_gravity_affects_body() {
        let mut world = PhysicsWorld::new();
        let transform = Transform::from_position(Vec3::new(0.0, 10.0, 0.0));
        let handle = world.create_dynamic_body(transform);
        world.add_box_collider(handle, Vec3::new(0.5, 0.5, 0.5));

        // Get initial position
        let initial_pos = world.get_transform(handle).unwrap().position;

        // Step simulation multiple times
        for _ in 0..60 {
            world.step();
        }

        // Position should have decreased due to gravity
        let final_pos = world.get_transform(handle).unwrap().position;
        assert!(final_pos.y < initial_pos.y, "Body should fall due to gravity");
    }

    #[test]
    fn test_static_body_doesnt_move() {
        let mut world = PhysicsWorld::new();
        let transform = Transform::from_position(Vec3::new(0.0, 0.0, 0.0));
        let handle = world.create_static_body(transform);
        world.add_box_collider(handle, Vec3::new(10.0, 0.5, 10.0));

        let initial_pos = world.get_transform(handle).unwrap().position;

        // Step simulation
        for _ in 0..60 {
            world.step();
        }

        let final_pos = world.get_transform(handle).unwrap().position;
        assert_eq!(initial_pos, final_pos, "Static body should not move");
    }

    #[test]
    fn test_velocity() {
        let mut world = PhysicsWorld::new();
        let transform = Transform::from_position(Vec3::new(0.0, 10.0, 0.0));
        let handle = world.create_dynamic_body(transform);
        world.add_sphere_collider(handle, 1.0);

        // Set velocity
        world.set_velocity(handle, Vec3::new(5.0, 0.0, 0.0));

        // Get velocity
        let velocity = world.get_velocity(handle).unwrap();
        assert_eq!(velocity, Vec3::new(5.0, 0.0, 0.0));

        // After stepping, velocity should change due to gravity
        world.step();
        let new_velocity = world.get_velocity(handle).unwrap();
        assert!(new_velocity.y < 0.0, "Y velocity should be negative due to gravity");
    }

    #[test]
    fn test_collision_detection() {
        let mut world = PhysicsWorld::new();

        // Create ground (static)
        let ground_transform = Transform::from_position(Vec3::new(0.0, -0.5, 0.0));
        let ground_handle = world.create_static_body(ground_transform);
        world.add_box_collider(ground_handle, Vec3::new(10.0, 0.5, 10.0));

        // Create falling box (dynamic)
        let box_transform = Transform::from_position(Vec3::new(0.0, 5.0, 0.0));
        let box_handle = world.create_dynamic_body(box_transform);
        world.add_box_collider(box_handle, Vec3::new(0.5, 0.5, 0.5));

        // Step simulation until box settles
        for _ in 0..120 {
            world.step();
        }

        // Box should have landed on ground
        // Ground top is at y=0, box half-height is 0.5, so box center should be around y=0.5
        // Allow some tolerance for physics settling
        let final_pos = world.get_transform(box_handle).unwrap().position;
        assert!(
            final_pos.y > 0.3 && final_pos.y < 1.0,
            "Box should rest on ground, y={}", final_pos.y
        );
    }
}
