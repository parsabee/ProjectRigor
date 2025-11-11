// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Cornell box scene - a classic ray tracing test scene.
//!
//! This module provides [`CornellBox`], a scene builder that creates the iconic
//! Cornell box used for testing ray tracing algorithms. The scene consists of
//! a room with colored walls and two boxes.

use crate::ecs::{DynamicTag, PhysicsBodyComponent, RenderComponent, StaticTag, TransformComponent};
use crate::math::Transform;
use crate::scene::{Scene, SceneBuilder};
use glam::{Quat, Vec3};

/// A Cornell box scene builder.
///
/// This scene builder creates the classic Cornell box:
/// - Red left wall
/// - Green right wall
/// - White back wall, floor, and ceiling
/// - Two white boxes in the center (one tall, one short)
///
/// The Cornell box is a standard test scene in computer graphics for
/// demonstrating global illumination, color bleeding, and ray tracing.
///
/// # Example
///
/// ```rust,no_run
/// use projectrigor::scene::Scene;
/// use projectrigor::scenes::cornell_box::CornellBox;
/// use projectrigor::scene::SceneBuilder;
///
/// let mut scene = Scene::new();
/// CornellBox::build(&mut scene);
/// ```
pub struct CornellBox;

impl SceneBuilder for CornellBox {
    /// Builds the Cornell box scene.
    ///
    /// Creates all walls, floor, ceiling, and the two boxes inside.
    fn build(scene: &mut Scene) {
        const ROOM_SIZE: f32 = 10.0;
        const WALL_THICKNESS: f32 = 0.1;

        // Left wall (red)
        let left_wall_transform = Transform {
            position: Vec3::new(-ROOM_SIZE / 2.0, ROOM_SIZE / 2.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(WALL_THICKNESS, ROOM_SIZE, ROOM_SIZE),
        };
        let left_wall_body = scene.physics.create_static_body(left_wall_transform);
        scene.physics.add_box_collider(left_wall_body, Vec3::new(WALL_THICKNESS, ROOM_SIZE, ROOM_SIZE));
        scene.world.spawn((
            TransformComponent { transform: left_wall_transform },
            PhysicsBodyComponent::new(left_wall_body),
            RenderComponent::cube([0.8, 0.1, 0.1]), // Red
            StaticTag,
        ));

        // Right wall (green)
        let right_wall_transform = Transform {
            position: Vec3::new(ROOM_SIZE / 2.0, ROOM_SIZE / 2.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(WALL_THICKNESS, ROOM_SIZE, ROOM_SIZE),
        };
        let right_wall_body = scene.physics.create_static_body(right_wall_transform);
        scene.physics.add_box_collider(right_wall_body, Vec3::new(WALL_THICKNESS, ROOM_SIZE, ROOM_SIZE));
        scene.world.spawn((
            TransformComponent { transform: right_wall_transform },
            PhysicsBodyComponent::new(right_wall_body),
            RenderComponent::cube([0.1, 0.8, 0.1]), // Green
            StaticTag,
        ));

        // Back wall (white)
        let back_wall_transform = Transform {
            position: Vec3::new(0.0, ROOM_SIZE / 2.0, -ROOM_SIZE / 2.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(ROOM_SIZE, ROOM_SIZE, WALL_THICKNESS),
        };
        let back_wall_body = scene.physics.create_static_body(back_wall_transform);
        scene.physics.add_box_collider(back_wall_body, Vec3::new(ROOM_SIZE, ROOM_SIZE, WALL_THICKNESS));
        scene.world.spawn((
            TransformComponent { transform: back_wall_transform },
            PhysicsBodyComponent::new(back_wall_body),
            RenderComponent::cube([0.8, 0.8, 0.8]), // White
            StaticTag,
        ));

        // Floor (white)
        let floor_transform = Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(ROOM_SIZE, WALL_THICKNESS, ROOM_SIZE),
        };
        let floor_body = scene.physics.create_static_body(floor_transform);
        scene.physics.add_box_collider(floor_body, Vec3::new(ROOM_SIZE, WALL_THICKNESS, ROOM_SIZE));
        scene.world.spawn((
            TransformComponent { transform: floor_transform },
            PhysicsBodyComponent::new(floor_body),
            RenderComponent::cube([0.8, 0.8, 0.8]), // White
            StaticTag,
        ));

        // Ceiling (white)
        let ceiling_transform = Transform {
            position: Vec3::new(0.0, ROOM_SIZE, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(ROOM_SIZE, WALL_THICKNESS, ROOM_SIZE),
        };
        let ceiling_body = scene.physics.create_static_body(ceiling_transform);
        scene.physics.add_box_collider(ceiling_body, Vec3::new(ROOM_SIZE, WALL_THICKNESS, ROOM_SIZE));
        scene.world.spawn((
            TransformComponent { transform: ceiling_transform },
            PhysicsBodyComponent::new(ceiling_body),
            RenderComponent::cube([0.8, 0.8, 0.8]), // White
            StaticTag,
        ));

        // Tall box (white) - positioned on the left side of the floor
        let tall_box_transform = Transform {
            position: Vec3::new(-2.0, 2.0, -2.0), // Y=2.0 sits on floor (height 4.0)
            rotation: Quat::from_rotation_y(0.3),
            scale: Vec3::new(1.5, 4.0, 1.5),
        };
        let tall_box_body = scene.physics.create_static_body(tall_box_transform);
        scene.physics.add_box_collider(tall_box_body, Vec3::new(1.5, 4.0, 1.5));
        scene.world.spawn((
            TransformComponent { transform: tall_box_transform },
            PhysicsBodyComponent::new(tall_box_body),
            RenderComponent::cube([0.8, 0.8, 0.8]), // White
            StaticTag,
        ));

        // Short box (white) - positioned on the right side of the floor
        let short_box_transform = Transform {
            position: Vec3::new(2.0, 1.0, -1.5), // Y=1.0 sits on floor (height 2.0)
            rotation: Quat::from_rotation_y(-0.4),
            scale: Vec3::new(1.5, 2.0, 1.5),
        };
        let short_box_body = scene.physics.create_static_body(short_box_transform);
        scene.physics.add_box_collider(short_box_body, Vec3::new(1.5, 2.0, 1.5));
        scene.world.spawn((
            TransformComponent { transform: short_box_transform },
            PhysicsBodyComponent::new(short_box_body),
            RenderComponent::cube([0.8, 0.8, 0.8]), // White
            StaticTag,
        ));

        // Dynamic sphere in the center (to make ray tracing visible)
        let sphere_transform = Transform::from_position(Vec3::new(0.0, 5.0, 1.0));
        let sphere_body = scene.physics.create_dynamic_body(sphere_transform);
        scene.physics.add_sphere_collider(sphere_body, 0.5); // Match demo scene radius
        scene.world.spawn((
            TransformComponent::new(sphere_transform),
            PhysicsBodyComponent::new(sphere_body),
            RenderComponent::sphere([1.0, 0.6, 0.2]), // Orange like demo scene
            DynamicTag,
        ));
    }
}
