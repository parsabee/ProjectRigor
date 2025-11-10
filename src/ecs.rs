// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Entity Component System (ECS) components and tags.
//!
//! This module defines the core components used in the ECS architecture:
//! - Transform components for position, rotation, and scale
//! - Physics body components linking entities to the physics simulation
//! - Render components specifying visual appearance (color and shape)
//! - Tag components for categorizing entities (static vs dynamic)
//!
//! The ECS uses the [`hecs`] library for efficient entity-component storage and queries.

use crate::math::Transform;
use glam::Vec3;
use rapier3d::prelude::RigidBodyHandle;

/// A component that stores an entity's spatial transform.
///
/// This component wraps a [`Transform`] which contains position, rotation, and scale.
/// It's used by the rendering system to determine where to draw objects and by the
/// physics system to synchronize entity positions with physics bodies.
///
/// # Example
///
/// ```rust
/// use project_rigor::ecs::TransformComponent;
/// use project_rigor::math::Transform;
/// use glam::Vec3;
///
/// let transform_comp = TransformComponent::from_position(Vec3::new(1.0, 2.0, 3.0));
/// assert_eq!(transform_comp.transform.position, Vec3::new(1.0, 2.0, 3.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    /// The transform data (position, rotation, scale)
    pub transform: Transform,
}

impl TransformComponent {
    /// Creates a new transform component from an existing transform.
    ///
    /// # Arguments
    ///
    /// * `transform` - The transform to wrap
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    /// Creates a new transform component at the specified position.
    ///
    /// The transform will have default rotation (identity) and scale (1, 1, 1).
    ///
    /// # Arguments
    ///
    /// * `position` - The position in 3D space
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::ecs::TransformComponent;
    /// use glam::Vec3;
    ///
    /// let transform = TransformComponent::from_position(Vec3::new(5.0, 0.0, -3.0));
    /// assert_eq!(transform.transform.position, Vec3::new(5.0, 0.0, -3.0));
    /// ```
    pub fn from_position(position: Vec3) -> Self {
        Self {
            transform: Transform::from_position(position),
        }
    }
}

/// A component that links an ECS entity to a physics rigid body.
///
/// This component stores a handle to a rigid body in the physics simulation.
/// It allows the physics system to update entity transforms based on physics
/// calculations.
///
/// # Example
///
/// ```rust,no_run
/// use project_rigor::ecs::PhysicsBodyComponent;
/// use rapier3d::prelude::RigidBodyHandle;
///
/// let handle = RigidBodyHandle::from_raw_parts(0, 1);
/// let physics_comp = PhysicsBodyComponent::new(handle);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct PhysicsBodyComponent {
    /// The handle to the rigid body in the physics world
    pub handle: RigidBodyHandle,
}

impl PhysicsBodyComponent {
    /// Creates a new physics body component.
    ///
    /// # Arguments
    ///
    /// * `handle` - The rigid body handle from the physics simulation
    pub fn new(handle: RigidBodyHandle) -> Self {
        Self { handle }
    }
}

/// A component that specifies how an entity should be rendered.
///
/// This component contains the visual appearance of an entity, including its
/// color (RGB values in range 0.0-1.0) and shape (cube or sphere).
///
/// # Example
///
/// ```rust
/// use project_rigor::ecs::RenderComponent;
///
/// let red_cube = RenderComponent::cube([1.0, 0.0, 0.0]);
/// let blue_sphere = RenderComponent::sphere([0.0, 0.0, 1.0]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RenderComponent {
    /// RGB color values (range 0.0-1.0)
    pub color: [f32; 3],
    /// The shape to render
    pub shape: RenderShape,
}

impl RenderComponent {
    /// Creates a new render component with the specified color and shape.
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color values (range 0.0-1.0)
    /// * `shape` - The shape to render
    pub fn new(color: [f32; 3], shape: RenderShape) -> Self {
        Self { color, shape }
    }

    /// Creates a render component for a cube with the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color values (range 0.0-1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::ecs::RenderComponent;
    ///
    /// let red_cube = RenderComponent::cube([1.0, 0.0, 0.0]);
    /// ```
    pub fn cube(color: [f32; 3]) -> Self {
        Self {
            color,
            shape: RenderShape::Cube,
        }
    }

    /// Creates a render component for a sphere with the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color values (range 0.0-1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// use project_rigor::ecs::RenderComponent;
    ///
    /// let green_sphere = RenderComponent::sphere([0.0, 1.0, 0.0]);
    /// ```
    pub fn sphere(color: [f32; 3]) -> Self {
        Self {
            color,
            shape: RenderShape::Sphere,
        }
    }
}

/// The types of shapes that can be rendered.
///
/// Currently supports cubes and spheres. Each shape has different geometry
/// and rendering characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderShape {
    /// A cube (1×1×1 unit box by default, scaled by transform)
    Cube,
    /// A sphere (radius 0.5 by default, scaled by transform)
    Sphere,
}

/// A tag component marking an entity as static (non-moving).
///
/// Static entities don't have their transforms updated by the physics system.
/// They're typically used for ground planes, walls, and other fixed geometry.
///
/// # Example
///
/// ```rust,no_run
/// use project_rigor::ecs::{TransformComponent, RenderComponent, StaticTag};
/// use glam::Vec3;
/// use hecs::World;
///
/// let mut world = World::new();
/// world.spawn((
///     TransformComponent::from_position(Vec3::ZERO),
///     RenderComponent::cube([0.5, 0.5, 0.5]),
///     StaticTag,
/// ));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StaticTag;

/// A tag component marking an entity as dynamic (physics-controlled).
///
/// Dynamic entities have their transforms updated from the physics simulation
/// each frame. They respond to forces, collisions, and gravity.
///
/// # Example
///
/// ```rust,no_run
/// use project_rigor::ecs::{TransformComponent, RenderComponent, PhysicsBodyComponent, DynamicTag};
/// use glam::Vec3;
/// use hecs::World;
/// use rapier3d::prelude::RigidBodyHandle;
///
/// let mut world = World::new();
/// let handle = RigidBodyHandle::from_raw_parts(0, 1);
/// world.spawn((
///     TransformComponent::from_position(Vec3::new(0.0, 5.0, 0.0)),
///     PhysicsBodyComponent::new(handle),
///     RenderComponent::cube([1.0, 0.0, 0.0]),
///     DynamicTag,
/// ));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DynamicTag;

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;
    use hecs::World;

    #[test]
    fn test_create_entity_with_components() {
        let mut world = World::new();
        
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let render = RenderComponent::cube([1.0, 0.0, 0.0]);
        
        let entity = world.spawn((
            TransformComponent::new(transform),
            render,
            DynamicTag,
        ));
        
        assert!(world.contains(entity));
    }

    #[test]
    fn test_query_transforms() {
        let mut world = World::new();
        
        // Create multiple entities with transforms
        world.spawn((
            TransformComponent::from_position(Vec3::new(0.0, 0.0, 0.0)),
            RenderComponent::cube([1.0, 0.0, 0.0]),
        ));
        
        world.spawn((
            TransformComponent::from_position(Vec3::new(1.0, 1.0, 1.0)),
            RenderComponent::cube([0.0, 1.0, 0.0]),
        ));
        
        world.spawn((
            TransformComponent::from_position(Vec3::new(2.0, 2.0, 2.0)),
            RenderComponent::cube([0.0, 0.0, 1.0]),
        ));
        
        // Query all entities with transform and render components
        let mut count = 0;
        for (_entity, (transform, render)) in world.query::<(&TransformComponent, &RenderComponent)>().iter() {
            count += 1;
            assert!(transform.transform.position.length() >= 0.0);
            assert_eq!(render.shape, RenderShape::Cube);
        }
        
        assert_eq!(count, 3);
    }

    #[test]
    fn test_query_dynamic_entities() {
        let mut world = World::new();
        
        // Static entity
        world.spawn((
            TransformComponent::from_position(Vec3::ZERO),
            RenderComponent::cube([0.5, 0.5, 0.5]),
            StaticTag,
        ));
        
        // Dynamic entities
        world.spawn((
            TransformComponent::from_position(Vec3::new(1.0, 0.0, 0.0)),
            RenderComponent::cube([1.0, 0.0, 0.0]),
            DynamicTag,
        ));
        
        world.spawn((
            TransformComponent::from_position(Vec3::new(2.0, 0.0, 0.0)),
            RenderComponent::cube([0.0, 1.0, 0.0]),
            DynamicTag,
        ));
        
        // Query only dynamic entities
        let mut dynamic_count = 0;
        for (_entity, (_transform, _tag)) in world.query::<(&TransformComponent, &DynamicTag)>().iter() {
            dynamic_count += 1;
        }
        
        assert_eq!(dynamic_count, 2);
    }

    #[test]
    fn test_modify_transform() {
        let mut world = World::new();
        
        let entity = world.spawn((
            TransformComponent::from_position(Vec3::ZERO),
            RenderComponent::cube([1.0, 1.0, 1.0]),
        ));
        
        // Modify the transform
        if let Ok(mut transform) = world.get::<&mut TransformComponent>(entity) {
            transform.transform.position = Vec3::new(5.0, 10.0, 15.0);
        }
        
        // Verify modification
        let pos = world.get::<&TransformComponent>(entity)
            .map(|t| t.transform.position)
            .unwrap();
        assert_eq!(pos, Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn test_physics_body_component() {
        let mut world = World::new();
        
        // Create a dummy physics body handle
        let handle = RigidBodyHandle::from_raw_parts(0, 1);
        
        let _entity = world.spawn((
            TransformComponent::from_position(Vec3::new(1.0, 2.0, 3.0)),
            PhysicsBodyComponent::new(handle),
            RenderComponent::cube([1.0, 0.0, 0.0]),
            DynamicTag,
        ));
        
        // Query entities with physics bodies
        let mut found = false;
        for (_entity, (physics, _transform)) in world.query::<(&PhysicsBodyComponent, &TransformComponent)>().iter() {
            assert_eq!(physics.handle, handle);
            found = true;
        }
        
        assert!(found);
    }

    #[test]
    fn test_remove_entity() {
        let mut world = World::new();
        
        let entity = world.spawn((
            TransformComponent::from_position(Vec3::ZERO),
            RenderComponent::cube([1.0, 1.0, 1.0]),
        ));
        
        assert!(world.contains(entity));
        
        world.despawn(entity).unwrap();
        
        assert!(!world.contains(entity));
    }

    #[test]
    fn test_render_shape_variants() {
        let cube = RenderComponent::cube([1.0, 0.0, 0.0]);
        let sphere = RenderComponent::sphere([0.0, 1.0, 0.0]);
        
        assert_eq!(cube.shape, RenderShape::Cube);
        assert_eq!(sphere.shape, RenderShape::Sphere);
        assert_ne!(cube.shape, sphere.shape);
    }

    #[test]
    fn test_query_by_color() {
        let mut world = World::new();
        
        let red = [1.0, 0.0, 0.0];
        let green = [0.0, 1.0, 0.0];
        
        world.spawn((
            TransformComponent::from_position(Vec3::ZERO),
            RenderComponent::cube(red),
        ));
        
        world.spawn((
            TransformComponent::from_position(Vec3::new(1.0, 0.0, 0.0)),
            RenderComponent::cube(green),
        ));
        
        // Count red objects
        let mut red_count = 0;
        for (_entity, render) in world.query::<&RenderComponent>().iter() {
            if render.color == red {
                red_count += 1;
            }
        }
        
        assert_eq!(red_count, 1);
    }
}
