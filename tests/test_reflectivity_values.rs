// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Test demonstrating different reflectivity values on triangles

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

#[test]
fn test_different_reflectivity_values() {
    let mut triangles = Vec::new();
    let light = LightUniforms::default();
    
    // Create three walls with different reflectivity values
    
    // Matte wall (reflectivity = 0.0) - no reflections
    let p0 = [-5.0, -5.0, 0.0];
    let p1 = [5.0, -5.0, 0.0];
    let p2 = [5.0, 5.0, 0.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0], // Red
        aabb_min,
        aabb_max,
        reflectivity: 0.0, // Completely matte
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Semi-reflective wall (reflectivity = 0.5) - moderate reflections
    let p0 = [-10.0, -5.0, -5.0];
    let p1 = [-10.0, 5.0, -5.0];
    let p2 = [-10.0, 5.0, 5.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [1.0, 0.0, 0.0],
        n1: [1.0, 0.0, 0.0],
        n2: [1.0, 0.0, 0.0],
        color: [0.0, 1.0, 0.0], // Green
        aabb_min,
        aabb_max,
        reflectivity: 0.5, // Semi-reflective
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Mirror wall (reflectivity = 0.9) - highly reflective
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [5.0, -5.0, -10.0];
    let p2 = [5.0, 5.0, -10.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [0.0, 0.0, 1.0], // Blue
        aabb_min,
        aabb_max,
        reflectivity: 0.9, // Nearly mirror-like
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Test ray pointing at the matte wall
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    
    let color_matte = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    
    // For matte surface, depth 1 and depth 3 should be nearly identical
    // (no reflections to compute)
    let color_matte_depth1 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 1);
    let diff_matte = (color_matte - color_matte_depth1).length();
    
    println!("Matte wall (reflectivity=0.0):");
    println!("  Depth 1: {:?}", color_matte_depth1);
    println!("  Depth 3: {:?}", color_matte);
    println!("  Difference: {}", diff_matte);
    
    // Matte surface should have minimal difference between depth levels
    assert!(diff_matte < 0.05, "Matte surface should not benefit from deeper traces");
    
    // The matte wall should be predominantly red
    assert!(color_matte.x > 0.3, "Matte wall should be reddish");
    
    println!("\nTest passed: Per-triangle reflectivity works correctly!");
    println!("  - Matte surfaces (reflectivity=0.0) don't compute reflections");
    println!("  - Each triangle can have its own material properties");
}

#[test]
fn test_reflectivity_range_validation() {
    // Verify that different reflectivity values produce expected behavior
    let mut triangles = Vec::new();
    let light = LightUniforms::default();
    
    // Back wall (to reflect off of)
    let p0 = [-5.0, -5.0, -10.0];
    let p1 = [5.0, -5.0, -10.0];
    let p2 = [5.0, 5.0, -10.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [0.0, 1.0, 0.0], // Green
        aabb_min,
        aabb_max,
        reflectivity: 0.0,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    
    // Front wall with varying reflectivity
    let p0 = [-5.0, -5.0, 0.0];
    let p1 = [5.0, -5.0, 0.0];
    let p2 = [5.0, 5.0, 0.0];
    let (aabb_min, aabb_max) = calculate_aabb(&p0, &p1, &p2);
    
    let ray_origin = Vec3::new(0.0, 0.0, 5.0);
    let ray_dir = Vec3::new(0.0, 0.0, -1.0);
    
    // Test with reflectivity = 0.0 (matte)
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0], // Red
        aabb_min,
        aabb_max,
        reflectivity: 0.0,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    let color_no_reflection = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    triangles.pop();
    
    // Test with reflectivity = 0.5 (semi-reflective)
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0], // Red
        aabb_min,
        aabb_max,
        reflectivity: 0.5,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    let color_semi_reflection = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    triangles.pop();
    
    // Test with reflectivity = 1.0 (perfect mirror)
    triangles.push(Triangle {
        p0, p1, p2,
        n0: [0.0, 0.0, 1.0],
        n1: [0.0, 0.0, 1.0],
        n2: [0.0, 0.0, 1.0],
        color: [1.0, 0.0, 0.0], // Red
        aabb_min,
        aabb_max,
        reflectivity: 1.0,
        _padding1: 0.0,
        _padding2: 0.0,
        _padding3: 0.0,
    });
    let color_full_reflection = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3);
    
    println!("\nReflectivity comparison:");
    println!("  0.0 (matte):    {:?}", color_no_reflection);
    println!("  0.5 (semi):     {:?}", color_semi_reflection);
    println!("  1.0 (mirror):   {:?}", color_full_reflection);
    
    // Matte should be mostly red (its base color)
    assert!(color_no_reflection.x > color_no_reflection.y, 
            "Matte surface should be predominantly red");
    
    // Semi-reflective should have less red than matte (some is replaced by reflection)
    assert!(color_semi_reflection.x < color_no_reflection.x,
            "Semi-reflective should have less base color due to reflection mixing");
    
    // Perfect mirror should have the least base color (most replaced by reflection)
    assert!(color_full_reflection.x < color_semi_reflection.x,
            "Mirror should have the least base color");
    
    println!("\nReflectivity range validation passed!");
}
