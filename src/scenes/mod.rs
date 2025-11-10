// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Pre-built scene configurations for the engine.
//!
//! This module contains various scene builders that implement the [`SceneBuilder`](crate::scene::SceneBuilder)
//! trait. Each scene builder creates a different configuration of entities, physics objects,
//! and visual elements.
//!
//! # Available Scenes
//!
//! - [`demo_scene::DemoScene`] - A demonstration scene with falling cubes and spheres

pub mod demo_scene;
