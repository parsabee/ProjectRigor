// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Integration tests to validate CPU and GPU ray tracers produce identical results.

use glam::Vec3;
use projectrigor::raytracer::trace_ray;
use projectrigor::scene::{LightUniforms, Triangle};

fn calculate_aabb(p0: &[f32; 3], p1: &[f32; 3], p2: &[f32; 3]) -> ([f32; 3], [f32; 3]) {
    let min = [
        p0[0].min(p1[0]).min(p2[0]),
        p0[1].min(p1[1]).min(p2[1]),
        p0[2].min(p1[2]).min(p2[2]),
    ];
    let max = [
        p0[0].max(p1[0]).max(p2[0]),
        p0[1].max(p1[1]).max(p2[1]),
        p0[2].max(p1[2]).max(p2[2]),
    ];
    (min, max)
}

/// Create a simple test scene with a few triangles
fn create_test_scene() -> Vec<Triangle> {
    let mut triangles = Vec::new();
    
    // Red wall (front) - triangle 1
    let p0 = [-5.0, -5.0, 0.0];
    let p1 = [5.0, -5.0, 0.0];
    let p2 = [5.0, 5.0, 0.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Red wall (front) - triangle 2
    let p0 = [-5.0, -5.0, 0.0];
    let p1 = [5.0, 5.0, 0.0];
    let p2 = [-5.0, 5.0, 0.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Green wall (back) - triangle 1
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [5.0, 5.0, -10.0];
    let p2 = [5.0, -5.0, -10.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, -1.0],
        n1: [0.0, 0.0, -1.0],
        n2: [0.0, 0.0, -1.0],
        color: [0.0, 1.0, 0.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Green wall (back) - triangle 2
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [-5.0, 5.0, -10.0];
    let p2 = [5.0, 5.0, -10.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, -1.0],
        n1: [0.0, 0.0, -1.0],
        n2: [0.0, 0.0, -1.0],
        color: [0.0, 1.0, 0.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Blue floor - triangle 1
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [5.0, -5.0, 0.0];
    let p2 = [5.0, -5.0, -10.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 1.0, 0.0],
        n1: [0.0, 1.0, 0.0],
        n2: [0.0, 1.0, 0.0],
        color: [0.0, 0.0, 1.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Blue floor - triangle 2
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [-5.0, -5.0, 0.0];
    let p2 = [5.0, -5.0, 0.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 1.0, 0.0],
        n1: [0.0, 1.0, 0.0],
        n2: [0.0, 1.0, 0.0],
        color: [0.0, 0.0, 1.0],
        aabb_min,
        aabb_max,
        reflectivity: 0.15,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    triangles
}

#[test]
fn test_cpu_single_ray() {
    let triangles = create_test_scene();
    let light = LightUniforms::default();
    
    // Test a ray pointing at the red wall
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    
    let color_depth2 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2);
    let color_depth3 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    
    println!("CPU depth 2: {:?}", color_depth2);
    println!("CPU depth 3: {:?}", color_depth3);
    
    // Both should produce red-ish color
    assert!(color_depth2.x > 0.3, "Depth 2 should be reddish");
    assert!(color_depth3.x > 0.3, "Depth 3 should be reddish");
    
    // They should be similar (allowing for floating point differences)
    let diff = (color_depth2 - color_depth3).length();
    assert!(diff < 0.1, "Depth 2 and 3 should produce similar results, diff: {}", diff);
}

#[test]
fn test_cpu_reflection_ray() {
    let triangles = create_test_scene();
    let light = LightUniforms::default();
    
    // Test a reflection scenario: ray that bounces off red wall toward green wall
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    // Ray at 45 degrees to hit red wall and reflect toward green
    let ray_dir = Vec3::new(0.0, 0.3, -1.0).normalize();
    
    let color_depth1 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 1);
    let color_depth2 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2);
    let color_depth3 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    
    println!("Reflection - depth 1: {:?}", color_depth1);
    println!("Reflection - depth 2: {:?}", color_depth2);
    println!("Reflection - depth 3: {:?}", color_depth3);
    
    // Depth 1: no reflections, should be pure local color
    // Depth 2 and 3: should include reflections, may have greenish tint
    assert!(color_depth1.length() > 0.1, "Depth 1 should have color");
    assert!(color_depth2.length() > 0.1, "Depth 2 should have color");
    assert!(color_depth3.length() > 0.1, "Depth 3 should have color - THIS IS THE BUG");
}

#[test]
fn test_cpu_max_depth_edge_cases() {
    let triangles = create_test_scene();
    let light = LightUniforms::default();
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    
    // Test various max depths
    for max_depth in 1..=5 {
        let color = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, max_depth);
        println!("Max depth {}: {:?}", max_depth, color);
        assert!(color.length() > 0.1, "Max depth {} should produce color", max_depth);
    }
}

/// Helper to compare two colors with a tolerance
#[allow(dead_code)]
fn colors_match(c1: Vec3, c2: Vec3, tolerance: f32) -> bool {
    (c1 - c2).length() < tolerance
}

#[test]
fn test_gpu_vs_cpu_comparison() {
    use metal::{Device, MTLPixelFormat, MTLResourceOptions};
    use std::mem;
    
    // Create Metal device (skip on non-Mac systems)
    let device = match Device::system_default() {
        Some(d) => d,
        None => {
            println!("No Metal device available - skipping GPU test");
            return;
        }
    };
    
    println!("=== GPU vs CPU Ray Tracing Comparison ===");
    println!("Metal device: {}", device.name());
    
    let triangles = create_test_scene();
    let light = LightUniforms::default();
    
    // Test basic ray tracing with a simple direct ray
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    let ray_target = ray_origin + ray_dir;
    
    // CPU ground truth
    println!("\n--- CPU Ground Truth (direct ray) ---");
    let cpu_depth1 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 1);
    let cpu_depth2 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2);
    let cpu_depth3 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    
    println!("CPU depth 1: {:?}", cpu_depth1);
    println!("CPU depth 2: {:?}", cpu_depth2);
    println!("CPU depth 3: {:?}", cpu_depth3);
    
    // Setup GPU infrastructure
    let command_queue = device.new_command_queue();
    
    let library_source = include_str!("../shaders/raytracing.metal");
    let library = device.new_library_with_source(library_source, &metal::CompileOptions::new())
        .expect("Failed to compile shader");
    
    let raytrace_function = library.get_function("raytrace_kernel", None)
        .expect("Failed to find raytrace_kernel");
    let pipeline = device.new_compute_pipeline_state_with_function(&raytrace_function)
        .expect("Failed to create pipeline");
    
    let light_buffer = device.new_buffer_with_data(
        &light as *const _ as *const _,
        mem::size_of::<LightUniforms>() as u64,
        MTLResourceOptions::StorageModeShared,
    );
    
    // Helper to trace on GPU
    let trace_gpu = |max_depth: i32| -> Vec3 {
        let texture_descriptor = metal::TextureDescriptor::new();
        texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA32Float);
        texture_descriptor.set_width(1);
        texture_descriptor.set_height(1);
        texture_descriptor.set_usage(metal::MTLTextureUsage::ShaderWrite | metal::MTLTextureUsage::ShaderRead);
        texture_descriptor.set_storage_mode(metal::MTLStorageMode::Shared);
        
        let output_texture = device.new_texture(&texture_descriptor);
        
        let triangle_buffer = device.new_buffer_with_data(
            triangles.as_ptr() as *const _,
            (triangles.len() * mem::size_of::<Triangle>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        #[repr(C)]
        struct RayTracingParams {
            camera_position: [f32; 3],
            aspect_ratio: f32,
            camera_target: [f32; 3],
            max_depth: i32,
            background_color: [f32; 3],
            default_reflectivity: f32,
            camera_forward: [f32; 3],
            _padding1: f32,
            camera_right: [f32; 3],
            _padding2: f32,
            camera_up: [f32; 3],
            _padding3: f32,
        }
        
        // Calculate camera basis
        let camera_forward = (ray_target - ray_origin).normalize();
        let camera_right = camera_forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let camera_up = camera_right.cross(camera_forward);
        
        let params = RayTracingParams {
            camera_position: ray_origin.to_array(),
            aspect_ratio: 1.0,
            camera_target: ray_target.to_array(),
            max_depth,
            background_color: [0.2, 0.0, 0.2],
            default_reflectivity: 0.15,
            camera_forward: camera_forward.to_array(),
            _padding1: 0.0,
            camera_right: camera_right.to_array(),
            _padding2: 0.0,
            camera_up: camera_up.to_array(),
            _padding3: 0.0,
        };
        
        let params_buffer = device.new_buffer_with_data(
            &params as *const _ as *const _,
            mem::size_of::<RayTracingParams>() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        // Triangle count buffer
        let triangle_count = triangles.len() as u32;
        let count_buffer = device.new_buffer_with_data(
            &triangle_count as *const u32 as *const _,
            mem::size_of::<u32>() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        
        let command_buffer = command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();
        
        encoder.set_compute_pipeline_state(&pipeline);
        encoder.set_texture(0, Some(&output_texture));
        encoder.set_buffer(0, Some(&triangle_buffer), 0);  // buffer(0) = triangles
        encoder.set_buffer(1, Some(&params_buffer), 0);    // buffer(1) = params
        encoder.set_buffer(2, Some(&light_buffer), 0);     // buffer(2) = light
        encoder.set_buffer(3, Some(&count_buffer), 0);     // buffer(3) = triangle_count
        
        let thread_group_size = metal::MTLSize::new(8, 8, 1);
        let thread_groups = metal::MTLSize::new(1, 1, 1);
        encoder.dispatch_thread_groups(thread_groups, thread_group_size);
        encoder.end_encoding();
        
        command_buffer.commit();
        command_buffer.wait_until_completed();
        
        let mut result = [0.0f32; 4];
        let bytes_per_row = 1 * 4 * mem::size_of::<f32>() as u64;
        let region = metal::MTLRegion::new_2d(0, 0, 1, 1);
        output_texture.get_bytes(result.as_mut_ptr() as *mut _, bytes_per_row, region, 0);
        
        Vec3::new(result[0], result[1], result[2])
    };
    
    // GPU results
    println!("\n--- GPU Results ---");
    println!("Testing GPU depth 1...");
    let gpu_depth1 = trace_gpu(1);
    println!("GPU depth 1: {:?}", gpu_depth1);
    
    println!("Testing GPU depth 2...");
    let gpu_depth2 = trace_gpu(2);
    println!("GPU depth 2: {:?}", gpu_depth2);
    
    println!("Testing GPU depth 3...");
    let gpu_depth3 = trace_gpu(3);
    println!("GPU depth 3: {:?}", gpu_depth3);
    
    // Validate basic GPU functionality
    println!("\n--- Validation ---");
    
    // Depth 1 should work (no reflections)
    assert!(gpu_depth1.length() > 0.01, "GPU depth 1 should produce color");
    assert!(gpu_depth1.x > gpu_depth1.y && gpu_depth1.x > gpu_depth1.z, 
            "GPU depth 1 should be reddish");
    println!("✓ GPU depth 1 works correctly (no reflections)");
    
    // Compare with CPU ground truth
    println!("\n--- Bug Detection ---");
    let depth2_matches = gpu_depth2.x > 0.3 && (gpu_depth2.x - cpu_depth2.x).abs() < 0.2;
    let depth3_matches = gpu_depth3.x > 0.3 && (gpu_depth3.x - cpu_depth3.x).abs() < 0.2;
    
    if !depth2_matches {
        println!("⚠️  BUG DETECTED at depth 2!");
        println!("   CPU (ground truth): {:?}", cpu_depth2);
        println!("   GPU (actual):       {:?}", gpu_depth2);
        println!("   Expected reddish color, got background/ambient");
    }
    
    if !depth3_matches {
        println!("⚠️  BUG DETECTED at depth 3!");
        println!("   CPU (ground truth): {:?}", cpu_depth3);
        println!("   GPU (actual):       {:?}", gpu_depth3);
        println!("   Expected reddish color, got background/ambient");
    }
    
    if !depth2_matches || !depth3_matches {
        panic!("GPU raytracer produces incorrect colors at depth 2 or 3 (reflection bug)");
    }
    
    println!("✓ GPU matches CPU ground truth at all depths");
    println!("✓ All reflection depths work correctly");
}

#[test]
fn test_depth_condition_logic() {
    // Test the core condition: depth < MAX_DEPTH - 1
    // This determines if we should reflect
    
    let test_cases = vec![
        (0, 2, true),   // depth 0, max_depth 2 -> should reflect
        (1, 2, false),  // depth 1, max_depth 2 -> should NOT reflect
        (0, 3, true),   // depth 0, max_depth 3 -> should reflect
        (1, 3, true),   // depth 1, max_depth 3 -> should reflect
        (2, 3, false),  // depth 2, max_depth 3 -> should NOT reflect
        (3, 3, false),  // depth 3, max_depth 3 -> early return (depth >= max_depth)
    ];
    
    for (depth, max_depth, should_reflect) in test_cases {
        // Early exit condition
        let should_exit = depth >= max_depth;
        // Reflection condition
        let can_reflect = !should_exit && (depth < max_depth - 1);
        
        println!(
            "depth={}, max_depth={}: exit={}, reflect={} (expected reflect: {})",
            depth, max_depth, should_exit, can_reflect, should_reflect
        );
        
        assert_eq!(
            can_reflect, should_reflect,
            "depth={}, max_depth={} reflection logic mismatch",
            depth, max_depth
        );
    }
}
