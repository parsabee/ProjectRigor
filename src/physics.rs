// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::math::Transform;
use glam::{Quat, Vec3};
use rapier3d::prelude::*;
use rapier3d::na::{Quaternion, UnitQuaternion};

/// Wrapper around rapier's physics world
pub struct PhysicsWorld {
    pub gravity: Vec3,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    /// Create a new physics world with default gravity
    pub fn new() -> Self {
        Self::with_gravity(Vec3::new(0.0, -9.81, 0.0))
    }

    /// Create a physics world with custom gravity
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

    /// Step the simulation forward by one time step
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

    /// Create a dynamic rigid body (affected by forces)
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

    /// Create a static rigid body (immovable)
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

    /// Add a box collider to a rigid body
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

    /// Add a sphere collider to a rigid body
    pub fn add_sphere_collider(
        &mut self,
        body_handle: RigidBodyHandle,
        radius: f32,
    ) -> ColliderHandle {
        let collider = ColliderBuilder::ball(radius).build();

        self.collider_set
            .insert_with_parent(collider, body_handle, &mut self.rigid_body_set)
    }

    /// Get the transform of a rigid body
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

    /// Set the velocity of a rigid body
    pub fn set_velocity(&mut self, handle: RigidBodyHandle, velocity: Vec3) {
        if let Some(body) = self.rigid_body_set.get_mut(handle) {
            body.set_linvel(vector![velocity.x, velocity.y, velocity.z], true);
        }
    }

    /// Get the velocity of a rigid body
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
