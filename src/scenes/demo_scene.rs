// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Demo scene featuring falling cubes and spheres with physics.
//!
//! This module provides [`DemoScene`], a demonstration scene builder that creates
//! a ground plane with falling colored cubes and spheres to showcase the physics
//! and rendering capabilities of the engine.

use crate::ecs::{DynamicTag, PhysicsBodyComponent, RenderComponent, StaticTag, TransformComponent};
use crate::math::Transform;
use crate::scene::{Scene, SceneBuilder};
use glam::Vec3;

/// A demonstration scene with falling cubes and spheres.
///
/// This scene builder creates:
/// - A large static ground plane (50×0.1×50 units) at y=-2
/// - 5 colored cubes falling from staggered heights
/// - 3 colored spheres falling from higher positions
///
/// The objects are spread out in 3D space to demonstrate collision detection,
/// physics simulation, and rendering with different shapes and colors.
///
/// # Example
///
/// ```rust,no_run
/// use project_rigor::scene::Scene;
/// use project_rigor::scenes::demo_scene::DemoScene;
/// use project_rigor::scene::SceneBuilder;
///
/// let mut scene = Scene::new();
/// DemoScene::build(&mut scene);
/// ```
pub struct DemoScene;

impl SceneBuilder for DemoScene {
    /// Builds the demo scene by creating ground, cubes, and spheres.
    ///
    /// This creates:
    /// - 1 static ground plane
    /// - 5 dynamic cubes (red, green, blue, yellow, magenta)
    /// - 3 dynamic spheres (cyan, orange, purple)
    fn build(scene: &mut Scene) {
        // Create ground plane
        Self::create_ground(scene);
        
        // Create falling objects
        Self::create_falling_cubes(scene, 5);
        Self::create_falling_spheres(scene, 3);
    }
}

impl DemoScene {
    /// Creates a static ground plane for objects to fall onto.
    ///
    /// The ground is a large flat box (50×0.1×50 units) positioned at y=-2.0.
    /// It has a gray color and is marked as static so it doesn't move.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to add the ground plane to
    fn create_ground(scene: &mut Scene) {
        let mut ground_transform = Transform::from_position(Vec3::new(0.0, -2.0, 0.0));
        ground_transform.scale = Vec3::new(50.0, 0.1, 50.0);
        let ground_body = scene.physics.create_static_body(ground_transform);
        scene.physics.add_box_collider(ground_body, Vec3::new(50.0, 0.1, 50.0));
        
        scene.world.spawn((
            TransformComponent::new(ground_transform),
            PhysicsBodyComponent::new(ground_body),
            RenderComponent::cube([0.3, 0.3, 0.3]), // Gray ground
            StaticTag,
        ));
        
        println!("Created ground at y=-2.0");
    }

    /// Creates a specified number of falling cubes with different colors.
    ///
    /// The cubes are positioned at increasing X coordinates (spaced 3 units apart)
    /// and staggered heights, creating a cascading effect as they fall.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to add the cubes to
    /// * `count` - Number of cubes to create (colors cycle through red, green, blue, yellow, magenta)
    ///
    /// # Cube Properties
    ///
    /// - Size: 1×1×1 unit cubes
    /// - Spacing: 3 units apart along X axis
    /// - Height: Starting at y=2.0, increasing by 2.2 units per cube
    /// - Physics: Dynamic rigid bodies with box colliders
    fn create_falling_cubes(scene: &mut Scene, count: usize) {
        let colors = [
            [1.0, 0.3, 0.3], // Red
            [0.3, 1.0, 0.3], // Green
            [0.3, 0.3, 1.0], // Blue
            [1.0, 1.0, 0.3], // Yellow
            [1.0, 0.3, 1.0], // Magenta
        ];
        
        for i in 0..count {
            let x = (i as f32) * 3.0;
            let y = 2.0 + (i as f32) * 2.2;
            let transform = Transform::from_position(Vec3::new(x, y, 0.0));
            let body = scene.physics.create_dynamic_body(transform);
            scene.physics.add_box_collider(body, Vec3::new(1.0, 1.0, 1.0));
            
            scene.world.spawn((
                TransformComponent::new(transform),
                PhysicsBodyComponent::new(body),
                RenderComponent::cube(colors[i % colors.len()]),
                DynamicTag,
            ));
            
            println!("Created box {} at position ({}, {}, 0.0)", i, x, y);
        }
    }

    /// Creates a specified number of falling spheres with different colors.
    ///
    /// The spheres are positioned at increasing X coordinates (spaced 3 units apart),
    /// offset in Z (-3 units) to separate them from the cubes, and at higher starting
    /// heights than the cubes.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to add the spheres to
    /// * `count` - Number of spheres to create (colors cycle through cyan, orange, purple)
    ///
    /// # Sphere Properties
    ///
    /// - Radius: 0.5 units
    /// - Spacing: 3 units apart along X axis
    /// - Height: Starting at y=8.0, increasing by 2.5 units per sphere
    /// - Z offset: -3 units (to separate from cubes)
    /// - Physics: Dynamic rigid bodies with sphere colliders
    fn create_falling_spheres(scene: &mut Scene, count: usize) {
        let sphere_colors = [
            [0.3, 1.0, 1.0], // Cyan
            [1.0, 0.6, 0.2], // Orange
            [0.6, 0.3, 1.0], // Purple
        ];
        
        for i in 0..count {
            let x = (i as f32) * 3.0;
            let y = 8.0 + (i as f32) * 2.5; // Start higher than cubes
            let z = -3.0; // Offset in Z to separate from cubes
            let transform = Transform::from_position(Vec3::new(x, y, z));
            let body = scene.physics.create_dynamic_body(transform);
            scene.physics.add_sphere_collider(body, 0.5); // Radius 0.5 to match visual sphere
            
            scene.world.spawn((
                TransformComponent::new(transform),
                PhysicsBodyComponent::new(body),
                RenderComponent::sphere(sphere_colors[i % sphere_colors.len()]),
                DynamicTag,
            ));
            
            println!("Created sphere {} at position ({}, {}, {})", i, x, y, z);
        }
    }
}
