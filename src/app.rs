// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Application state and event handling.
//!
//! This module contains the [`App`] struct which manages the main application state,
//! including the window, renderer, scene, and input handling. It implements the
//! `winit::ApplicationHandler` trait for event-driven updates.

use crate::renderer::MetalRenderer;
use crate::scene::{Scene, SceneBuilder};
use crate::scenes::demo_scene::DemoScene;
use std::collections::HashSet;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};

/// Main application state managing window, renderer, scene, and input.
///
/// The `App` struct coordinates all the major systems:
/// - **Window**: The OS window (via winit)
/// - **Renderer**: Metal-based graphics rendering
/// - **Scene**: ECS world and physics simulation
/// - **Input**: Keyboard and mouse input tracking for camera control
///
/// # Input Controls
///
/// - **W/S**: Move camera forward/backward
/// - **A/D**: Move camera left/right
/// - **Q/E**: Move camera down/up
/// - **Mouse drag**: Rotate camera (horizontal and vertical)
///
/// # Example
///
/// ```rust,no_run
/// use projectrigor::App;
/// use winit::event_loop::EventLoop;
///
/// let event_loop = EventLoop::new().unwrap();
/// let mut app = App::new();
/// event_loop.run_app(&mut app).unwrap();
/// ```
pub struct App {
    window: Option<Window>,
    renderer: Option<MetalRenderer>,
    scene: Scene,
    frame_count: u32,
    pressed_keys: HashSet<KeyCode>,
    camera_speed: f32,
    mouse_pressed: bool,
    last_mouse_pos: Option<(f64, f64)>,
    mouse_sensitivity: f32,
}

impl App {
    /// Creates a new application instance with the demo scene.
    ///
    /// This initializes:
    /// - An empty scene populated with the demo content (falling cubes and spheres)
    /// - Camera controls with default speed (0.2 units/frame) and mouse sensitivity (0.005 rad/pixel)
    /// - Input state tracking
    ///
    /// The window and renderer are created lazily when the event loop starts.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use projectrigor::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        let mut scene = Scene::new();
        DemoScene::build(&mut scene);
        
        Self {
            window: None,
            renderer: None,
            scene,
            frame_count: 0,
            pressed_keys: HashSet::new(),
            camera_speed: 0.2, // Units per frame
            mouse_pressed: false,
            last_mouse_pos: None,
            mouse_sensitivity: 0.005, // Radians per pixel
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
            WindowEvent::KeyboardInput { event, .. } => {
                // Track pressed keys
                if let winit::keyboard::PhysicalKey::Code(keycode) = event.physical_key {
                    if event.state.is_pressed() {
                        self.pressed_keys.insert(keycode);
                    } else {
                        self.pressed_keys.remove(&keycode);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // Track mouse button state for drag
                if button == MouseButton::Left {
                    self.mouse_pressed = state == ElementState::Pressed;
                    if !self.mouse_pressed {
                        // Reset last position when button released
                        self.last_mouse_pos = None;
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Handle mouse drag for camera rotation
                if self.mouse_pressed {
                    if let Some(last_pos) = self.last_mouse_pos {
                        let delta_x = position.x - last_pos.0;
                        let delta_y = position.y - last_pos.1;
                        
                        // Apply rotation to camera
                        if let Some(renderer) = &mut self.renderer {
                            let camera = renderer.camera_mut();
                            let sensitivity = self.mouse_sensitivity;
                            
                            // Horizontal rotation (yaw)
                            camera.rotate_horizontal(-(delta_x as f32) * sensitivity);
                            
                            // Vertical rotation (pitch)
                            camera.rotate_vertical(-(delta_y as f32) * sensitivity);
                        }
                    }
                    self.last_mouse_pos = Some((position.x, position.y));
                }
            }
            WindowEvent::Resized(_) => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) {
                    renderer.resize(window);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) {
                    // Update camera position based on pressed keys
                    let camera = renderer.camera_mut();
                    let speed = self.camera_speed;
                    
                    if self.pressed_keys.contains(&KeyCode::KeyW) {
                        camera.move_forward(speed);
                    }
                    if self.pressed_keys.contains(&KeyCode::KeyS) {
                        camera.move_backward(speed);
                    }
                    if self.pressed_keys.contains(&KeyCode::KeyA) {
                        camera.move_left(speed);
                    }
                    if self.pressed_keys.contains(&KeyCode::KeyD) {
                        camera.move_right(speed);
                    }
                    if self.pressed_keys.contains(&KeyCode::KeyQ) {
                        camera.move_down(speed);
                    }
                    if self.pressed_keys.contains(&KeyCode::KeyE) {
                        camera.move_up(speed);
                    }
                    
                    // Step physics simulation and update transforms
                    self.scene.step_physics();
                    
                    // Get render data from scene
                    // Debug: print first dynamic entity position every 60 frames
                    if self.frame_count % 60 == 0 {
                        if let Some(pos) = self.scene.get_first_dynamic_position() {
                            println!("Frame {}: First dynamic entity at {:?}", self.frame_count, pos);
                        }
                    }
                    
                    renderer.render(&self.scene);
                    self.frame_count += 1;
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
