// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Scene management system for the physics-based robotics engine.
//!
//! This module provides the core [`Scene`] struct that manages the Entity Component System (ECS)
//! world and physics simulation, along with the [`SceneBuilder`] trait for creating custom scenes.
//!
//! # Architecture
//!
//! The scene system separates concerns between:
//! - **Scene**: Manages the lifecycle of entities and physics simulation
//! - **SceneBuilder**: Creates and populates scenes with entities
//! - **App**: Handles windowing, input, and rendering
//!
//! # Example
//!
//! ```rust,no_run
//! use projectrigor::scene::{Scene, SceneBuilder};
//! use projectrigor::scenes::demo_scene::DemoScene;
//!
//! let mut scene = Scene::new();
//! DemoScene::build(&mut scene);
//!
//! // In game loop:
//! scene.step_physics();
//! let render_data = scene.get_render_data();
//! ```

use crate::ecs::{DynamicTag, PhysicsBodyComponent, RenderComponent, TransformComponent};
use crate::math::Transform;
use crate::physics::PhysicsWorld;
use hecs::World;

/// A scene that manages an ECS world and physics simulation.
///
/// The `Scene` struct owns both the [`hecs::World`] for entity-component storage
/// and the [`PhysicsWorld`] for physics simulation. It provides a clean interface
/// for stepping physics and extracting render data.
///
/// # Fields
///
/// - `world`: The ECS world containing all entities and components
/// - `physics`: The physics simulation managing rigid bodies and colliders
///
/// # Example
///
/// ```rust,no_run
/// use projectrigor::scene::Scene;
///
/// let mut scene = Scene::new();
/// // Add entities to scene.world and scene.physics
/// scene.step_physics();
/// let render_data = scene.get_render_data();
/// ```
pub struct Scene {
    /// The ECS world containing all entities and their components
    pub world: World,
    /// The physics simulation world
    pub physics: PhysicsWorld,
}

impl Scene {
    /// Creates a new empty scene with an initialized ECS world and physics simulation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use projectrigor::scene::Scene;
    ///
    /// let scene = Scene::new();
    /// ```
    pub fn new() -> Self {
        Self {
            world: World::new(),
            physics: PhysicsWorld::new(),
        }
    }

    /// Steps the physics simulation forward by one timestep and synchronizes entity transforms.
    ///
    /// This method performs two operations:
    /// 1. Advances the physics simulation by calling [`PhysicsWorld::step`]
    /// 2. Updates all dynamic entities' transform components with their physics transforms
    ///
    /// Only entities marked with [`DynamicTag`] have their transforms updated from physics.
    /// Static entities maintain their original transforms.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use projectrigor::scene::Scene;
    /// let mut scene = Scene::new();
    /// // ... add entities to scene
    /// scene.step_physics(); // Advances physics by one frame
    /// ```
    pub fn step_physics(&mut self) {
        self.physics.step();
        
        // Update entity transforms from physics (only for dynamic entities)
        for (_entity, (transform_comp, physics_comp, _)) in self
            .world
            .query_mut::<(&mut TransformComponent, &PhysicsBodyComponent, &DynamicTag)>()
        {
            if let Some(physics_transform) = self.physics.get_transform(physics_comp.handle) {
                transform_comp.transform = physics_transform;
            }
        }
    }

    /// Extracts render data from all entities in the scene.
    ///
    /// Queries all entities with both [`TransformComponent`] and [`RenderComponent`]
    /// and returns their transform, color, and shape as a vector of tuples.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing:
    /// - [`Transform`]: The entity's position, rotation, and scale
    /// - `[f32; 3]`: The entity's RGB color (values in range 0.0-1.0)
    /// - [`crate::ecs::RenderShape`]: The entity's shape (Cube or Sphere)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use projectrigor::scene::Scene;
    /// let scene = Scene::new();
    /// let render_data = scene.get_render_data();
    /// // Pass render_data to renderer
    /// ```
    pub fn get_render_data(&self) -> Vec<(Transform, [f32; 3], crate::ecs::RenderShape)> {
        self.world
            .query::<(&TransformComponent, &RenderComponent)>()
            .iter()
            .map(|(_entity, (transform_comp, render_comp))| {
                (transform_comp.transform, render_comp.color, render_comp.shape.clone())
            })
            .collect()
    }

    /// Returns the position of the first dynamic entity found in the scene.
    ///
    /// This method is primarily used for debugging and testing. It queries for entities
    /// with both [`TransformComponent`] and [`DynamicTag`], returning the position of
    /// the first match.
    ///
    /// # Returns
    ///
    /// - `Some(Vec3)`: The position of the first dynamic entity
    /// - `None`: If no dynamic entities exist in the scene
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use projectrigor::scene::Scene;
    /// let scene = Scene::new();
    /// if let Some(pos) = scene.get_first_dynamic_position() {
    ///     println!("First dynamic entity at: {:?}", pos);
    /// }
    /// ```
    pub fn get_first_dynamic_position(&self) -> Option<glam::Vec3> {
        self.world
            .query::<(&TransformComponent, &DynamicTag)>()
            .iter()
            .next()
            .map(|(_entity, (transform_comp, _tag))| transform_comp.transform.position)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for building and populating scenes with entities.
///
/// Implement this trait to create reusable scene configurations. Scene builders
/// can create ground planes, populate the world with objects, set up lighting, etc.
///
/// # Example
///
/// ```rust,no_run
/// use projectrigor::scene::{Scene, SceneBuilder};
/// use projectrigor::ecs::{TransformComponent, RenderComponent, StaticTag};
/// use projectrigor::math::Transform;
/// use glam::Vec3;
///
/// pub struct EmptyScene;
///
/// impl SceneBuilder for EmptyScene {
///     fn build(scene: &mut Scene) {
///         // Create a simple ground plane
///         let transform = Transform::from_position(Vec3::new(0.0, -1.0, 0.0));
///         scene.world.spawn((
///             TransformComponent::new(transform),
///             RenderComponent::cube([0.5, 0.5, 0.5]),
///             StaticTag,
///         ));
///     }
/// }
/// ```
pub trait SceneBuilder {
    /// Populates the given scene with entities, physics bodies, and other scene setup.
    ///
    /// # Arguments
    ///
    /// * `scene` - The scene to populate with entities and physics objects
    fn build(scene: &mut Scene);
}
