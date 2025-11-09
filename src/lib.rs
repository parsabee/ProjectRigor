// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![allow(unexpected_cfgs)]

pub mod app;
pub mod camera;
pub mod ecs;
pub mod math;
pub mod physics;
pub mod renderer;

pub use app::App;
pub use camera::Camera;
pub use math::Transform;
pub use physics::PhysicsWorld;
pub use renderer::MetalRenderer;

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
