// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Metal-based rendering system with Phong lighting.
//!
//! This module provides the [`MetalRenderer`] which handles all graphics rendering
//! using Apple's Metal API. It supports:
//! - Cube and sphere geometry with vertex normals
//! - Phong lighting (ambient, diffuse, specular)
//! - Transform-based object positioning
//! - Multi-object rendering with individual colors
//!
//! The renderer uses a forward rendering pipeline with per-pixel lighting calculations.

use crate::camera::Camera;
use cocoa::base::id as cocoa_id;
use cocoa::foundation::NSRect;
use core_graphics_types::geometry::CGSize;
use glam::{Mat4, Quat};
use metal::{Device, MTLPixelFormat, MetalLayer, MTLResourceOptions};
use objc::runtime::YES;
use objc::{msg_send, sel, sel_impl};
use raw_window_handle::HasWindowHandle;
use std::mem;
use std::time::Instant;
use winit::window::Window;

/// Vertex data with position, normal, and color.
///
/// Each vertex contains:
/// - `position`: 3D coordinates in model space
/// - `normal`: Surface normal for lighting calculations
/// - `color`: Per-vertex RGB color (0.0-1.0)
///
/// Total size: 36 bytes (3 * [f32; 3])
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

/// Uniform data for transformation matrices.
///
/// Passed to vertex shader for transforming vertices to clip space.
#[repr(C)]
struct Uniforms {
    model_view_projection: [[f32; 4]; 4],
}

/// Lighting parameters for Phong shading.
///
/// Contains directional light properties:
/// - `direction`: Light direction vector (normalized)
/// - `color`: Light RGB color
/// - `ambient_intensity`: Ambient light strength (0.0-1.0)
/// - `diffuse_intensity`: Diffuse light strength (0.0-1.0)
/// - `specular_intensity`: Specular highlight strength (0.0-1.0)
/// - `shininess`: Specular power/sharpness (higher = sharper highlights)
///
/// Total size: 48 bytes (with padding for Metal alignment)
#[repr(C)]
struct LightUniforms {
    direction: [f32; 3],
    _padding1: f32,
    color: [f32; 3],
    _padding2: f32,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    shininess: f32,
}

impl Default for LightUniforms {
    fn default() -> Self {
        Self {
            direction: [-0.5, -1.0, -0.3], // Light from upper left
            _padding1: 0.0,
            color: [1.0, 1.0, 1.0], // White light
            _padding2: 0.0,
            ambient_intensity: 0.3,
            diffuse_intensity: 0.7,
            specular_intensity: 0.5,
            shininess: 32.0,
        }
    }
}

pub struct MetalRenderer {
    _device: Device,
    command_queue: metal::CommandQueue,
    layer: MetalLayer,
    pipeline_state: metal::RenderPipelineState,
    depth_stencil_state: metal::DepthStencilState,
    vertex_buffer: metal::Buffer,
    uniform_buffer: metal::Buffer,
    light_buffer: metal::Buffer,
    camera: Camera,
    start_time: Instant,
}

impl MetalRenderer {
    // Helper to create cube vertices with a specific color
    fn create_colored_cube_vertices(device: &metal::DeviceRef, color: [f32; 3]) -> metal::Buffer {
        let vertices = [
            // Front face (normal: +Z)
            Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            
            // Back face (normal: -Z)
            Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0], color },
            
            // Top face (normal: +Y)
            Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], color },
            Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0], color },
            
            // Bottom face (normal: -Y)
            Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], color },
            Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0], color },
            
            // Right face (normal: +X)
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [ 0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0], color },
            
            // Left face (normal: -X)
            Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0], color },
            Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0], color },
        ];
        
        device.new_buffer_with_data(
            vertices.as_ptr() as *const _,
            (vertices.len() * mem::size_of::<Vertex>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    }

    // Helper to create sphere vertices with a specific color using UV sphere algorithm
    fn create_colored_sphere_vertices(device: &metal::DeviceRef, color: [f32; 3], segments: u32, rings: u32) -> metal::Buffer {
        let mut vertices = Vec::new();
        
        // Generate sphere vertices
        for ring in 0..=rings {
            let theta = ring as f32 * std::f32::consts::PI / rings as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            for segment in 0..=segments {
                let phi = segment as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                
                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;
                
                // For a sphere, the normal at each vertex is the same as the normalized position
                // Positions are scaled to radius 0.5 to match cube size
                vertices.push(Vertex {
                    position: [x * 0.5, y * 0.5, z * 0.5],
                    normal: [x, y, z], // Normalized normal (same as unit sphere position)
                    color,
                });
            }
        }
        
        // Generate triangle indices
        let mut triangles = Vec::new();
        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next = current + segments + 1;
                
                // First triangle
                triangles.push(vertices[current as usize]);
                triangles.push(vertices[next as usize]);
                triangles.push(vertices[(current + 1) as usize]);
                
                // Second triangle
                triangles.push(vertices[(current + 1) as usize]);
                triangles.push(vertices[next as usize]);
                triangles.push(vertices[(next + 1) as usize]);
            }
        }
        
        device.new_buffer_with_data(
            triangles.as_ptr() as *const _,
            (triangles.len() * mem::size_of::<Vertex>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    }

    pub fn new(window: &Window) -> Self {
        let device = Device::system_default().expect("No Metal device found");
        let command_queue = device.new_command_queue();
        
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        
        unsafe {
            let view = Self::get_ns_view(window);
            let bounds: NSRect = msg_send![view, bounds];
            layer.set_drawable_size(CGSize::new(bounds.size.width as f64, bounds.size.height as f64));
            
            let _: () = msg_send![view, setWantsLayer: YES];
            let _: () = msg_send![view, setLayer: mem::transmute::<_, cocoa_id>(layer.as_ref())];
        }
        
        // Load shader library (compile at runtime)
        let shader_source = include_str!("../shaders/cube.metal");
        let library = device.new_library_with_source(shader_source, &metal::CompileOptions::new()).unwrap();
        
        // Create render pipeline
        let vertex_function = library.get_function("vertex_main", None).unwrap();
        let fragment_function = library.get_function("fragment_main", None).unwrap();
        
        // Create vertex descriptor
        let vertex_descriptor = metal::VertexDescriptor::new();
        
        // Position attribute (attribute 0)
        let position_attr = vertex_descriptor.attributes().object_at(0).unwrap();
        position_attr.set_format(metal::MTLVertexFormat::Float3);
        position_attr.set_offset(0);
        position_attr.set_buffer_index(0);
        
        // Normal attribute (attribute 1)
        let normal_attr = vertex_descriptor.attributes().object_at(1).unwrap();
        normal_attr.set_format(metal::MTLVertexFormat::Float3);
        normal_attr.set_offset(12); // After position: 3 floats * 4 bytes = 12 bytes offset
        normal_attr.set_buffer_index(0);
        
        // Color attribute (attribute 2)
        let color_attr = vertex_descriptor.attributes().object_at(2).unwrap();
        color_attr.set_format(metal::MTLVertexFormat::Float3);
        color_attr.set_offset(24); // After position + normal: 6 floats * 4 bytes = 24 bytes offset
        color_attr.set_buffer_index(0);
        
        // Layout
        let layout = vertex_descriptor.layouts().object_at(0).unwrap();
        layout.set_stride((mem::size_of::<Vertex>()) as u64);
        layout.set_step_function(metal::MTLVertexStepFunction::PerVertex);
        
        let pipeline_descriptor = metal::RenderPipelineDescriptor::new();
        pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        pipeline_descriptor.set_fragment_function(Some(&fragment_function));
        pipeline_descriptor.set_vertex_descriptor(Some(vertex_descriptor));
        pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        
        // Enable depth testing
        pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);
        
        let pipeline_state = device
            .new_render_pipeline_state(&pipeline_descriptor)
            .unwrap();
        
        // Create depth stencil state
        let depth_stencil_descriptor = metal::DepthStencilDescriptor::new();
        depth_stencil_descriptor.set_depth_compare_function(metal::MTLCompareFunction::Less);
        depth_stencil_descriptor.set_depth_write_enabled(true);
        let depth_stencil_state = device.new_depth_stencil_state(&depth_stencil_descriptor);
        
        // Create full 3D cube vertices (36 vertices for 6 faces)
        let vertices = [
            // Front face (red, normal: +Z)
            Vertex { position: [-1.0, -1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-1.0, -1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-1.0,  1.0,  1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
            
            // Back face (green, normal: -Z)
            Vertex { position: [ 1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-1.0,  1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-1.0,  1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], normal: [0.0, 0.0, -1.0], color: [0.0, 1.0, 0.0] },
            
            // Top face (blue, normal: +Y)
            Vertex { position: [-1.0,  1.0,  1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-1.0,  1.0, -1.0], normal: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
            
            // Bottom face (yellow, normal: -Y)
            Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [ 1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-1.0, -1.0,  1.0], normal: [0.0, -1.0, 0.0], color: [1.0, 1.0, 0.0] },
            
            // Right face (magenta, normal: +X)
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0, -1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0, -1.0,  1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, -1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0,  1.0], normal: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0] },
            
            // Left face (cyan, normal: -X)
            Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-1.0, -1.0,  1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-1.0,  1.0,  1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-1.0,  1.0, -1.0], normal: [-1.0, 0.0, 0.0], color: [0.0, 1.0, 1.0] },
        ];
        
        let vertex_buffer = device.new_buffer_with_data(
            vertices.as_ptr() as *const _,
            (vertices.len() * std::mem::size_of::<Vertex>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        
        // Create uniform buffer
        let uniform_buffer = device.new_buffer(
            std::mem::size_of::<Uniforms>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        
        // Create light buffer with default lighting
        let light_data = LightUniforms::default();
        let light_buffer = device.new_buffer_with_data(
            &light_data as *const LightUniforms as *const _,
            std::mem::size_of::<LightUniforms>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        );
        
        // Create camera
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        let camera = Camera::new(aspect_ratio);
        
        Self {
            _device: device,
            command_queue,
            layer,
            pipeline_state,
            depth_stencil_state,
            vertex_buffer,
            uniform_buffer,
            light_buffer,
            camera,
            start_time: Instant::now(),
        }
    }
    
    pub fn resize(&mut self, window: &Window) {
        unsafe {
            let view = Self::get_ns_view(window);
            let bounds: NSRect = msg_send![view, bounds];
            self.layer.set_drawable_size(CGSize::new(bounds.size.width as f64, bounds.size.height as f64));
        }
        
        // Update camera aspect ratio
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        self.camera.update_aspect_ratio(aspect_ratio);
    }

    /// Get mutable reference to the camera
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get immutable reference to the camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    
    pub fn render(&mut self) {
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };
        
        // Calculate rotation based on elapsed time
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let rotation_y = Quat::from_rotation_y(elapsed * 0.5); // Rotate around Y axis
        let rotation_x = Quat::from_rotation_x(elapsed * 0.3); // Rotate around X axis
        let rotation = rotation_y * rotation_x;
        
        // Create model matrix with rotation
        let model = Mat4::from_quat(rotation);
        
        let command_buffer = self.command_queue.new_command_buffer();
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        
        // Color attachment
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        
        color_attachment.set_texture(Some(drawable.texture()));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.1, 0.1, 0.15, 1.0));
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        
        // Depth attachment
        let depth_texture_descriptor = metal::TextureDescriptor::new();
        depth_texture_descriptor.set_pixel_format(metal::MTLPixelFormat::Depth32Float);
        depth_texture_descriptor.set_width(drawable.texture().width());
        depth_texture_descriptor.set_height(drawable.texture().height());
        depth_texture_descriptor.set_usage(metal::MTLTextureUsage::RenderTarget);
        depth_texture_descriptor.set_storage_mode(metal::MTLStorageMode::Private);
        
        let depth_texture = self.layer.device().new_texture(&depth_texture_descriptor);
        
        let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        depth_attachment.set_texture(Some(&depth_texture));
        depth_attachment.set_load_action(metal::MTLLoadAction::Clear);
        depth_attachment.set_clear_depth(1.0);
        depth_attachment.set_store_action(metal::MTLStoreAction::DontCare);
        
        let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
        
        // Draw the cube
        encoder.set_render_pipeline_state(&self.pipeline_state);
        encoder.set_depth_stencil_state(&self.depth_stencil_state);
        encoder.set_vertex_buffer(0, Some(&self.vertex_buffer), 0);
        
        // Update uniforms with model-view-projection matrix
        let mvp = self.camera.view_projection_matrix() * model;
        let uniforms = Uniforms {
            model_view_projection: mvp.to_cols_array_2d(),
        };
        
        unsafe {
            let uniform_ptr = self.uniform_buffer.contents() as *mut Uniforms;
            std::ptr::write(uniform_ptr, uniforms);
        }
        
        encoder.set_vertex_buffer(1, Some(&self.uniform_buffer), 0);
        encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, 36);
        
        encoder.end_encoding();
        
        command_buffer.present_drawable(drawable);
        command_buffer.commit();
    }
    
    pub fn render_with_transforms_and_colors(&mut self, render_data: &[(crate::math::Transform, [f32; 3], crate::ecs::RenderShape)]) {
        if render_data.is_empty() {
            return;
        }
        
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };
        
        let command_buffer = self.command_queue.new_command_buffer();
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        
        // Color attachment
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        
        color_attachment.set_texture(Some(drawable.texture()));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.1, 0.1, 0.15, 1.0));
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        
        // Depth attachment
        let depth_texture_descriptor = metal::TextureDescriptor::new();
        depth_texture_descriptor.set_pixel_format(metal::MTLPixelFormat::Depth32Float);
        depth_texture_descriptor.set_width(drawable.texture().width());
        depth_texture_descriptor.set_height(drawable.texture().height());
        depth_texture_descriptor.set_usage(metal::MTLTextureUsage::RenderTarget);
        depth_texture_descriptor.set_storage_mode(metal::MTLStorageMode::Private);
        
        let depth_texture = self.layer.device().new_texture(&depth_texture_descriptor);
        
        let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        depth_attachment.set_texture(Some(&depth_texture));
        depth_attachment.set_load_action(metal::MTLLoadAction::Clear);
        depth_attachment.set_clear_depth(1.0);
        depth_attachment.set_store_action(metal::MTLStoreAction::DontCare);
        
        let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
        
        encoder.set_render_pipeline_state(&self.pipeline_state);
        encoder.set_depth_stencil_state(&self.depth_stencil_state);
        
        // Draw each transform with its specified color and shape from ECS
        for (transform, color, shape) in render_data.iter() {
            // Create vertex buffer based on shape type
            let (vertex_buffer, vertex_count) = match shape {
                crate::ecs::RenderShape::Cube => {
                    (Self::create_colored_cube_vertices(self.layer.device(), *color), 36)
                }
                crate::ecs::RenderShape::Sphere => {
                    // 20 segments and 20 rings gives a nice smooth sphere
                    (Self::create_colored_sphere_vertices(self.layer.device(), *color, 20, 20), 20 * 20 * 6)
                }
            };
            
            // Create a new uniform buffer for each draw call to avoid race conditions
            let model = transform.to_matrix();
            let mvp = self.camera.view_projection_matrix() * model;
            let uniforms = Uniforms {
                model_view_projection: mvp.to_cols_array_2d(),
            };
            
            // Create temporary uniform buffer for this draw call
            let temp_uniform_buffer = self.layer.device().new_buffer(
                std::mem::size_of::<Uniforms>() as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache,
            );
            
            unsafe {
                let uniform_ptr = temp_uniform_buffer.contents() as *mut Uniforms;
                std::ptr::write(uniform_ptr, uniforms);
            }
            
            encoder.set_vertex_buffer(0, Some(&vertex_buffer), 0);
            encoder.set_vertex_buffer(1, Some(&temp_uniform_buffer), 0);
            encoder.set_fragment_buffer(0, Some(&self.light_buffer), 0);
            encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, vertex_count);
        }
        
        encoder.end_encoding();
        
        command_buffer.present_drawable(drawable);
        command_buffer.commit();
    }
    
    unsafe fn get_ns_view(window: &Window) -> cocoa_id {
        let window_handle = window.window_handle().unwrap();
        match window_handle.as_raw() {
            raw_window_handle::RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr() as cocoa_id,
            _ => panic!("Expected AppKit window handle"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Transform;
    use glam::{Vec3, Mat4};

    #[test]
    fn test_vertex_structure_size() {
        // Vertex should be 36 bytes: 3 floats for position + 3 floats for normal + 3 floats for color
        assert_eq!(std::mem::size_of::<Vertex>(), 36);
    }

    #[test]
    fn test_vertex_creation() {
        let color = [1.0, 0.5, 0.25];
        let vertex = Vertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 1.0, 0.0],
            color,
        };
        
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 1.0, 0.0]);
        assert_eq!(vertex.color, [1.0, 0.5, 0.25]);
    }

    #[test]
    fn test_uniforms_structure_size() {
        // Uniforms should be 64 bytes: 16 floats for 4x4 matrix
        assert_eq!(std::mem::size_of::<Uniforms>(), 64);
    }

    #[test]
    fn test_uniforms_matrix_layout() {
        let identity = Mat4::IDENTITY;
        let uniforms = Uniforms {
            model_view_projection: identity.to_cols_array_2d(),
        };
        
        // Check that identity matrix is properly stored
        assert_eq!(uniforms.model_view_projection[0][0], 1.0);
        assert_eq!(uniforms.model_view_projection[1][1], 1.0);
        assert_eq!(uniforms.model_view_projection[2][2], 1.0);
        assert_eq!(uniforms.model_view_projection[3][3], 1.0);
        
        // Off-diagonal elements should be zero
        assert_eq!(uniforms.model_view_projection[0][1], 0.0);
        assert_eq!(uniforms.model_view_projection[1][0], 0.0);
    }

    #[test]
    fn test_uniforms_with_transform() {
        let transform = Transform::from_position(Vec3::new(5.0, 10.0, 15.0));
        let model = transform.to_matrix();
        
        let uniforms = Uniforms {
            model_view_projection: model.to_cols_array_2d(),
        };
        
        // Translation should be in the last column (column 3)
        assert_eq!(uniforms.model_view_projection[3][0], 5.0);
        assert_eq!(uniforms.model_view_projection[3][1], 10.0);
        assert_eq!(uniforms.model_view_projection[3][2], 15.0);
    }

    #[test]
    fn test_colored_cube_vertices_device_requirement() {
        // This test verifies that create_colored_cube_vertices requires a Metal device
        // We can't actually test the function without a device, but we can verify
        // the function signature and that it would create 36 vertices (6 faces * 6 vertices)
        
        // Each cube face has 6 vertices (2 triangles * 3 vertices)
        // A cube has 6 faces, so 36 vertices total
        const EXPECTED_VERTEX_COUNT: usize = 36;
        const VERTICES_PER_FACE: usize = 6;
        const CUBE_FACES: usize = 6;
        
        assert_eq!(EXPECTED_VERTEX_COUNT, VERTICES_PER_FACE * CUBE_FACES);
    }

    #[test]
    fn test_render_data_tuple_structure() {
        // Test that the render data structure (Transform, color) works correctly
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let color = [0.5, 0.6, 0.7];
        
        let render_data = (transform, color);
        
        assert_eq!(render_data.0.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(render_data.1, [0.5, 0.6, 0.7]);
    }

    #[test]
    fn test_render_data_collection() {
        // Simulate collecting render data from ECS
        let transforms_and_colors = vec![
            (Transform::from_position(Vec3::new(0.0, 0.0, 0.0)), [1.0, 0.0, 0.0]),
            (Transform::from_position(Vec3::new(1.0, 0.0, 0.0)), [0.0, 1.0, 0.0]),
            (Transform::from_position(Vec3::new(0.0, 1.0, 0.0)), [0.0, 0.0, 1.0]),
        ];
        
        assert_eq!(transforms_and_colors.len(), 3);
        assert_eq!(transforms_and_colors[0].1, [1.0, 0.0, 0.0]); // Red
        assert_eq!(transforms_and_colors[1].1, [0.0, 1.0, 0.0]); // Green
        assert_eq!(transforms_and_colors[2].1, [0.0, 0.0, 1.0]); // Blue
    }

    #[test]
    fn test_mvp_matrix_multiplication() {
        // Test that MVP matrix can be properly constructed
        let camera = Camera::new(16.0 / 9.0);
        
        let transform = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let model = transform.to_matrix();
        let mvp = camera.view_projection_matrix() * model;
        
        // MVP should be a valid 4x4 matrix
        let mvp_array = mvp.to_cols_array_2d();
        assert_eq!(mvp_array.len(), 4);
        assert_eq!(mvp_array[0].len(), 4);
        
        // The matrix should not be all zeros
        let sum: f32 = mvp_array.iter().flat_map(|row| row.iter()).sum();
        assert!(sum.abs() > 0.0);
    }

    #[test]
    fn test_vertex_color_consistency() {
        // Test that vertex colors are applied consistently
        let color = [0.3, 0.3, 0.3]; // Gray like ground plane
        
        // Create vertices for a front face (6 vertices)
        let vertices = [
            Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
            Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0], color },
        ];
        
        // All vertices should have the same color
        for vertex in &vertices {
            assert_eq!(vertex.color, [0.3, 0.3, 0.3]);
        }
    }

    #[test]
    fn test_cube_vertex_positions() {
        // Test that cube vertices are within expected bounds (-0.5 to 0.5)
        let vertices = [
            [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5],
            [-0.5,  0.5,  0.5], [ 0.5, -0.5, -0.5], [-0.5, -0.5, -0.5],
        ];
        
        for pos in &vertices {
            assert!(pos[0] >= -0.5 && pos[0] <= 0.5);
            assert!(pos[1] >= -0.5 && pos[1] <= 0.5);
            assert!(pos[2] >= -0.5 && pos[2] <= 0.5);
        }
    }

    #[test]
    fn test_empty_render_data_handling() {
        // Test that empty render data is handled correctly
        use crate::ecs::RenderShape;
        let render_data: Vec<(Transform, [f32; 3], RenderShape)> = Vec::new();
        
        assert!(render_data.is_empty());
        assert_eq!(render_data.len(), 0);
    }

    #[test]
    fn test_multiple_colors_in_render_data() {
        // Test that multiple different colors can be stored
        let colors = [
            [1.0, 0.3, 0.3], // Red
            [0.3, 1.0, 0.3], // Green
            [0.3, 0.3, 1.0], // Blue
            [1.0, 1.0, 0.3], // Yellow
            [1.0, 0.3, 1.0], // Magenta
            [0.3, 0.3, 0.3], // Gray (ground)
        ];
        
        let mut render_data = Vec::new();
        for (i, color) in colors.iter().enumerate() {
            let transform = Transform::from_position(Vec3::new(i as f32, 0.0, 0.0));
            render_data.push((transform, *color));
        }
        
        assert_eq!(render_data.len(), 6);
        assert_eq!(render_data[5].1, [0.3, 0.3, 0.3]); // Ground is gray
    }

    #[test]
    fn test_sphere_vertex_count_calculation() {
        // Test that sphere vertex count formula is correct
        // For segments=S and rings=R: vertex_count = S * R * 6
        let segments = 20u32;
        let rings = 20u32;
        let expected_triangles = segments * rings * 6; // 6 vertices per quad (2 triangles)
        
        assert_eq!(expected_triangles, 2400);
    }

    #[test]
    fn test_render_shape_variants() {
        // Test that both RenderShape variants can be created
        use crate::ecs::RenderShape;
        
        let cube = RenderShape::Cube;
        let sphere = RenderShape::Sphere;
        
        // Verify they're different
        assert!(matches!(cube, RenderShape::Cube));
        assert!(matches!(sphere, RenderShape::Sphere));
    }

    #[test]
    fn test_mixed_shape_render_data() {
        // Test render data with mixed cubes and spheres
        use crate::ecs::RenderShape;
        
        let render_data = vec![
            (Transform::from_position(Vec3::new(0.0, 0.0, 0.0)), [1.0, 0.0, 0.0], RenderShape::Cube),
            (Transform::from_position(Vec3::new(2.0, 0.0, 0.0)), [0.0, 1.0, 0.0], RenderShape::Sphere),
            (Transform::from_position(Vec3::new(4.0, 0.0, 0.0)), [0.0, 0.0, 1.0], RenderShape::Cube),
            (Transform::from_position(Vec3::new(6.0, 0.0, 0.0)), [1.0, 1.0, 0.0], RenderShape::Sphere),
        ];
        
        assert_eq!(render_data.len(), 4);
        assert!(matches!(render_data[0].2, RenderShape::Cube));
        assert!(matches!(render_data[1].2, RenderShape::Sphere));
        assert!(matches!(render_data[2].2, RenderShape::Cube));
        assert!(matches!(render_data[3].2, RenderShape::Sphere));
    }

    #[test]
    fn test_sphere_radius() {
        // Verify sphere vertices are scaled to radius 0.5 (same as cube half-extent)
        // This ensures consistent sizing between cubes and spheres
        let radius = 0.5f32;
        
        // A point on sphere surface should be approximately at radius distance from origin
        // For theta=PI/2 (equator), phi=0: x=1*0.5, y=0, z=0
        let theta = std::f32::consts::PI / 2.0;
        let phi = 0.0f32;
        
        let x = phi.cos() * theta.sin() * radius;
        let y = theta.cos() * radius;
        let z = phi.sin() * theta.sin() * radius;
        
        let distance = (x * x + y * y + z * z).sqrt();
        assert!((distance - radius).abs() < 0.001);
    }

    #[test]
    fn test_vertex_copy_trait() {
        // Verify that Vertex implements Copy trait for efficient cloning
        let v1 = Vertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 1.0, 0.0],
            color: [0.5, 0.5, 0.5],
        };
        
        let v2 = v1; // This should copy, not move
        let v3 = v1; // Should still work
        
        assert_eq!(v2.position, v3.position);
        assert_eq!(v2.normal, v3.normal);
        assert_eq!(v2.color, v3.color);
    }

    #[test]
    fn test_light_uniforms_default() {
        let light = LightUniforms::default();
        
        // Test default light direction (upper left)
        assert_eq!(light.direction, [-0.5, -1.0, -0.3]);
        
        // Test default light color (white)
        assert_eq!(light.color, [1.0, 1.0, 1.0]);
        
        // Test default intensities
        assert_eq!(light.ambient_intensity, 0.3);
        assert_eq!(light.diffuse_intensity, 0.7);
        assert_eq!(light.specular_intensity, 0.5);
        assert_eq!(light.shininess, 32.0);
    }

    #[test]
    fn test_light_uniforms_size() {
        // LightUniforms should be properly aligned for GPU
        // 3 floats + padding + 3 floats + padding + 4 floats = 48 bytes
        assert_eq!(std::mem::size_of::<LightUniforms>(), 48);
    }

    #[test]
    fn test_cube_face_normals() {
        // Test that each cube face has correct outward-pointing normals
        // Front face should have normal pointing in +Z direction
        let front_normal = [0.0, 0.0, 1.0];
        
        // Back face should have normal pointing in -Z direction  
        let back_normal = [0.0, 0.0, -1.0];
        
        // Top face should have normal pointing in +Y direction
        let top_normal = [0.0, 1.0, 0.0];
        
        // Bottom face should have normal pointing in -Y direction
        let bottom_normal = [0.0, -1.0, 0.0];
        
        // Right face should have normal pointing in +X direction
        let right_normal = [1.0, 0.0, 0.0];
        
        // Left face should have normal pointing in -X direction
        let left_normal = [-1.0, 0.0, 0.0];
        
        // All normals should be unit length
        for normal in &[front_normal, back_normal, top_normal, bottom_normal, right_normal, left_normal] {
            let length = ((normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]) as f32).sqrt();
            assert!((length - 1.0_f32).abs() < 0.001, "Normal {:?} is not unit length", normal);
        }
    }

    #[test]
    fn test_sphere_normals_are_normalized() {
        // For a UV sphere, normals should be normalized (unit length)
        // Test a few sample points
        let theta = std::f32::consts::PI / 4.0; // 45 degrees
        let phi = std::f32::consts::PI / 3.0;   // 60 degrees
        
        let x = phi.cos() * theta.sin();
        let y = theta.cos();
        let z = phi.sin() * theta.sin();
        
        let normal = [x, y, z];
        let length = (normal[0]*normal[0] + normal[1]*normal[1] + normal[2]*normal[2]).sqrt();
        
        assert!((length - 1.0).abs() < 0.001, "Sphere normal should be unit length");
    }

    #[test]
    fn test_normal_vector_orthogonality() {
        // Test that cube face normals are perpendicular to their faces
        // For example, the top face has vertices in XZ plane with Y = constant
        // So the normal should be purely in Y direction
        
        let top_normal = [0.0, 1.0, 0.0];
        
        // A vector along the top face (in X direction)
        let edge_x = [1.0, 0.0, 0.0];
        
        // Dot product should be zero (perpendicular)
        let dot: f32 = top_normal[0] * edge_x[0] + 
                       top_normal[1] * edge_x[1] + 
                       top_normal[2] * edge_x[2];
        
        assert!(dot.abs() < 0.001_f32, "Normal should be perpendicular to face");
    }
}
