// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! ProjectRigor: A physics-based robotics and animation engine for Apple Silicon.
//!
//! ProjectRigor is a Rust-based engine designed for robotics simulation and animation,
//! leveraging Metal for hardware-accelerated rendering on Apple Silicon. It combines:
//! - Entity Component System (ECS) architecture using [`hecs`]
//! - 3D physics simulation using [`rapier3d`]
//! - Metal-based rendering with Phong lighting
//! - Interactive camera controls
//! - Modular scene system
//!
//! # Architecture
//!
//! The engine is organized into several key modules:
//!
//! - [`app`] - Application state and event handling
//! - [`scene`] - Scene management (ECS world + physics)
//! - [`scenes`] - Pre-built scene configurations
//! - [`ecs`] - Entity-component definitions
//! - [`physics`] - Physics simulation wrapper
//! - [`renderer`] - Metal-based rendering
//! - [`camera`] - 3D camera with controls
//! - [`math`] - 3D transform mathematics
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use projectrigor::App;
//! use winit::event_loop::EventLoop;
//!
//! fn main() {
//!     let event_loop = EventLoop::new().unwrap();
//!     let mut app = App::new();
//!     event_loop.run_app(&mut app).unwrap();
//! }
//! ```
//!
//! # Creating Custom Scenes
//!
//! ```rust,no_run
//! use projectrigor::scene::{Scene, SceneBuilder};
//! use projectrigor::ecs::{TransformComponent, RenderComponent, DynamicTag, PhysicsBodyComponent};
//! use projectrigor::math::Transform;
//! use glam::Vec3;
//!
//! struct MyScene;
//!
//! impl SceneBuilder for MyScene {
//!     fn build(scene: &mut Scene) {
//!         // Create a falling cube
//!         let transform = Transform::from_position(Vec3::new(0.0, 5.0, 0.0));
//!         let body = scene.physics.create_dynamic_body(transform);
//!         scene.physics.add_box_collider(body, Vec3::new(0.5, 0.5, 0.5));
//!         
//!         scene.world.spawn((
//!             TransformComponent::new(transform),
//!             PhysicsBodyComponent::new(body),
//!             RenderComponent::cube([1.0, 0.0, 0.0]),
//!             DynamicTag,
//!         ));
//!     }
//! }
//! ```
//!
//! # Controls
//!
//! - **W/A/S/D** - Move camera forward/left/backward/right
//! - **Q/E** - Move camera down/up
//! - **Mouse drag** - Rotate camera view
//!
//! # Features
//!
//! - ✅ Phong lighting with ambient, diffuse, and specular components
//! - ✅ Physics simulation with rigid bodies and colliders
//! - ✅ Camera controls (WASD + mouse)
//! - ✅ ECS architecture for flexible entity management
//! - ✅ Modular scene system
//! - 🚧 Robot skeleton and animation (planned)
//! - 🚧 Inverse kinematics (planned)

#![allow(unexpected_cfgs)]

pub mod app;
pub mod camera;
pub mod ecs;
pub mod math;
pub mod perf;
pub mod physics;
pub mod raytracer;
pub mod renderer;
pub mod scene;
pub mod scenes;

pub use app::App;
pub use camera::Camera;
pub use math::Transform;
pub use physics::PhysicsWorld;
pub use raytracer::trace_ray;
pub use renderer::MetalRenderer;
pub use scene::{Scene, SceneBuilder};

// Re-export ECS components
pub use ecs::{
    DynamicTag, PhysicsBodyComponent, RenderComponent, RenderShape, StaticTag,
    TransformComponent,
};

// Re-export glam for external use
pub use glam;
// Re-export hecs for external use
pub use hecs;
// Re-export rapier for external use
pub use rapier3d;

/// Rendering mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingMode {
    /// Hardware-accelerated ray tracing using Metal's ray tracing API
    HardwareRayTracing,
    /// Software ray tracing (CPU-based)
    SoftwareRayTracing,
    /// Traditional rasterization pipeline
    Rasterization,
}
