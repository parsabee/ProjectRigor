// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::ecs::{DynamicTag, PhysicsBodyComponent, RenderComponent, StaticTag, TransformComponent};
use crate::math::Transform;
use crate::physics::PhysicsWorld;
use crate::renderer::MetalRenderer;
use glam::Vec3;
use hecs::World;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Window>,
    renderer: Option<MetalRenderer>,
    physics: PhysicsWorld,
    world: World,
    frame_count: u32,
}

impl App {
    pub fn new() -> Self {
        let mut physics = PhysicsWorld::new();
        let mut world = World::new();
        
        // Create ground plane with proper scale
        let mut ground_transform = Transform::from_position(Vec3::new(0.0, -2.0, 0.0));
        ground_transform.scale = Vec3::new(50.0, 0.1, 50.0); // Match the physics collider size
        let ground_body = physics.create_static_body(ground_transform);
        physics.add_box_collider(ground_body, Vec3::new(50.0, 0.1, 50.0));
        
        // Create ground entity in ECS
        world.spawn((
            TransformComponent::new(ground_transform),
            PhysicsBodyComponent::new(ground_body),
            RenderComponent::cube([0.3, 0.3, 0.3]), // Gray ground
            StaticTag,
        ));
        
        println!("Created ground at y=-2.0");
        
        // Define colors for each box
        let colors = [
            [1.0, 0.3, 0.3], // Red
            [0.3, 1.0, 0.3], // Green
            [0.3, 0.3, 1.0], // Blue
            [1.0, 1.0, 0.3], // Yellow
            [1.0, 0.3, 1.0], // Magenta
        ];
        
        // Create falling boxes
        for i in 0..5 {
            let x = (i as f32) * 3.0;
            let y = 2.0 + (i as f32) * 2.2;
            let transform = Transform::from_position(Vec3::new(x, y, 0.0));
            let body = physics.create_dynamic_body(transform);
            physics.add_box_collider(body, Vec3::new(1.0, 1.0, 1.0));
            
            // Create entity for this box
            world.spawn((
                TransformComponent::new(transform),
                PhysicsBodyComponent::new(body),
                RenderComponent::cube(colors[i]),
                DynamicTag,
            ));
            
            println!("Created box {} at position ({}, {}, 0.0)", i, x, y);
        }
        
        Self {
            window: None,
            renderer: None,
            physics,
            world,
            frame_count: 0,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("ProjectRigor - Metal")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
            
            let window = event_loop.create_window(window_attributes).unwrap();
            let renderer = MetalRenderer::new(&window);
            
            // Request initial redraw
            window.request_redraw();
            
            self.window = Some(window);
            self.renderer = Some(renderer);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested, exiting...");
                event_loop.exit();
            }
            WindowEvent::Resized(_) => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) {
                    renderer.resize(window);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) {
                    // Step physics simulation
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
                    
                    // Collect transforms and colors for rendering
                    let render_data: Vec<(Transform, [f32; 3])> = self
                        .world
                        .query::<(&TransformComponent, &RenderComponent)>()
                        .iter()
                        .map(|(_entity, (transform_comp, render_comp))| {
                            (transform_comp.transform, render_comp.color)
                        })
                        .collect();
                    
                    // Debug: print first dynamic entity position every 60 frames
                    if self.frame_count % 60 == 0 {
                        if let Some((_entity, (transform_comp, _tag))) = self
                            .world
                            .query::<(&TransformComponent, &DynamicTag)>()
                            .iter()
                            .next()
                        {
                            println!(
                                "Frame {}: First dynamic entity at {:?}",
                                self.frame_count, transform_comp.transform.position
                            );
                        }
                    }
                    
                    renderer.render_with_transforms_and_colors(&render_data);
                    self.frame_count += 1;
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
