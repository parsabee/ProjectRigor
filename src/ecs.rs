// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::math::Transform;
use glam::Vec3;
use rapier3d::prelude::RigidBodyHandle;

/// Component that holds transform data (position, rotation, scale)
#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub transform: Transform,
}

impl TransformComponent {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            transform: Transform::from_position(position),
        }
    }
}

/// Component that links an entity to a physics rigid body
#[derive(Debug, Clone, Copy)]
pub struct PhysicsBodyComponent {
    pub handle: RigidBodyHandle,
}

impl PhysicsBodyComponent {
    pub fn new(handle: RigidBodyHandle) -> Self {
        Self { handle }
    }
}

/// Component that holds rendering data
#[derive(Debug, Clone, Copy)]
pub struct RenderComponent {
    pub color: [f32; 3],
    pub shape: RenderShape,
}

impl RenderComponent {
    pub fn new(color: [f32; 3], shape: RenderShape) -> Self {
        Self { color, shape }
    }

    pub fn cube(color: [f32; 3]) -> Self {
        Self {
            color,
            shape: RenderShape::Cube,
        }
    }

    pub fn sphere(color: [f32; 3]) -> Self {
        Self {
            color,
            shape: RenderShape::Sphere,
        }
    }
}

/// Types of renderable shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderShape {
    Cube,
    Sphere,
}

/// Tag component to mark entities as static (non-moving)
#[derive(Debug, Clone, Copy)]
pub struct StaticTag;

/// Tag component to mark entities as dynamic (physics-controlled)
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
