// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! CPU-side ray tracer for testing and validation.
//! 
//! This module implements the same ray tracing logic as the Metal shader,
//! allowing us to write unit tests and debug issues before deploying to GPU.

use glam::Vec3;
use crate::scene::{Triangle, LightUniforms};

const EPSILON: f32 = 0.001;
const INTERSECTION_EPSILON: f32 = 0.000001;

/// Ray-triangle intersection using Möller-Trumbore algorithm.
/// 
/// Returns (t, u, v) barycentric coordinates if hit, None otherwise.
fn ray_triangle_intersection(
    ray_origin: Vec3,
    ray_dir: Vec3,
    tri: &Triangle,
) -> Option<(f32, f32, f32)> {
    let p0 = Vec3::from(tri.p0);
    let p1 = Vec3::from(tri.p1);
    let p2 = Vec3::from(tri.p2);
    
    let edge1 = p1 - p0;
    let edge2 = p2 - p0;
    
    let h = ray_dir.cross(edge2);
    let a = edge1.dot(h);
    
    if a.abs() < INTERSECTION_EPSILON {
        return None; // Ray is parallel
    }
    
    let f = 1.0 / a;
    let s = ray_origin - p0;
    let u = f * s.dot(h);
    
    if u < 0.0 || u > 1.0 {
        return None;
    }
    
    let q = s.cross(edge1);
    let v = f * ray_dir.dot(q);
    
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    
    let t = f * edge2.dot(q);
    
    if t > INTERSECTION_EPSILON {
        Some((t, u, v))
    } else {
        None
    }
}

/// Trace a ray through the scene and return the color.
/// 
/// This matches the Metal shader implementation for validation purposes.
pub fn trace_ray(
    ray_origin: Vec3,
    ray_dir: Vec3,
    triangles: &[Triangle],
    light: &LightUniforms,
    depth: i32,
    max_depth: i32,
    reflectivity: f32,
) -> Vec3 {
    if depth >= max_depth {
        return Vec3::ZERO;
    }
    
    // Find closest intersection
    let mut closest_t = f32::INFINITY;
    let mut closest_idx: Option<usize> = None;
    let mut closest_u = 0.0;
    let mut closest_v = 0.0;
    
    for (i, tri) in triangles.iter().enumerate() {
        if let Some((t, u, v)) = ray_triangle_intersection(ray_origin, ray_dir, tri) {
            if t < closest_t {
                closest_t = t;
                closest_idx = Some(i);
                closest_u = u;
                closest_v = v;
            }
        }
    }
    
    let Some(idx) = closest_idx else {
        // No intersection - return background color
        return Vec3::new(0.2, 0.0, 0.2); // dark purple
    };
    
    let tri = &triangles[idx];
    let w = 1.0 - closest_u - closest_v;
    
    // Interpolate normal
    let n0 = Vec3::from(tri.n0);
    let n1 = Vec3::from(tri.n1);
    let n2 = Vec3::from(tri.n2);
    let normal = (w * n0 + closest_u * n1 + closest_v * n2).normalize();
    
    // Calculate hit point
    let hit_point = ray_origin + ray_dir * closest_t;
    
    // Phong lighting
    let light_dir = Vec3::from(light.direction).normalize();
    let ndotl = normal.dot(-light_dir).max(0.0);
    
    // Ambient component
    let light_color = Vec3::from(light.color);
    let ambient = light.ambient_intensity * light_color;
    
    // Diffuse component
    let diffuse = light.diffuse_intensity * ndotl * light_color;
    
    // Specular component (Blinn-Phong)
    let mut specular = Vec3::ZERO;
    if ndotl > 0.0 {
        let view_dir = -ray_dir;
        let half_dir = (-light_dir + view_dir).normalize();
        let spec = normal.dot(half_dir).max(0.0).powf(light.shininess);
        specular = light.specular_intensity * spec * light_color;
    }
    
    // Combine lighting
    let lighting = ambient + diffuse + specular;
    let local_color = Vec3::from(tri.color) * lighting;
    
    // Trace reflections (if not at max depth)
    if depth < max_depth - 1 {
        let reflection_dir = ray_dir - 2.0 * ray_dir.dot(normal) * normal; // reflect()
        let reflection_origin = hit_point + normal * EPSILON;
        let reflection_color = trace_ray(
            reflection_origin,
            reflection_dir,
            triangles,
            light,
            depth + 1,
            max_depth,
            reflectivity,
        );
        
        // Blend local color with reflection
        return local_color.lerp(reflection_color, reflectivity);
    }
    
    local_color
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_simple_triangle(color: [f32; 3]) -> Triangle {
        Triangle {
            p0: [0.0, 0.0, 0.0],
            p1: [1.0, 0.0, 0.0],
            p2: [0.0, 1.0, 0.0],
            n0: [0.0, 0.0, 1.0],
            n1: [0.0, 0.0, 1.0],
            n2: [0.0, 0.0, 1.0],
            color,
        }
    }
    
    #[test]
    fn test_ray_miss() {
        let tri = make_simple_triangle([1.0, 0.0, 0.0]);
        let triangles = vec![tri];
        let light = LightUniforms::default();
        
        // Ray pointing away from triangle
        let ray_origin = Vec3::new(0.5, 0.5, 1.0);
        let ray_dir = Vec3::new(0.0, 0.0, 1.0); // pointing away
        
        let color = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2, 0.15);
        
        // Should return background color
        assert_eq!(color, Vec3::new(0.2, 0.0, 0.2));
    }
    
    #[test]
    fn test_ray_hit() {
        let tri = make_simple_triangle([1.0, 0.0, 0.0]); // red triangle
        let triangles = vec![tri];
        let light = LightUniforms::default();
        
        // Ray pointing at triangle
        let ray_origin = Vec3::new(0.5, 0.25, 1.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0);
        
        let color = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2, 0.15);
        
        // Should return some red color (with lighting)
        // Note: ambient light can add small amounts to other channels
        assert!(color.x > 0.0, "Should have red component");
        assert!(color.y < 0.1, "Should have minimal green");
        assert!(color.z < 0.1, "Should have minimal blue");
    }
    
    #[test]
    fn test_max_depth_reached() {
        let tri = make_simple_triangle([1.0, 1.0, 1.0]);
        let triangles = vec![tri];
        let light = LightUniforms::default();
        
        let ray_origin = Vec3::new(0.5, 0.25, 1.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0);
        
        // At max depth, should return black
        let color = trace_ray(ray_origin, ray_dir, &triangles, &light, 2, 2, 0.15);
        assert_eq!(color, Vec3::ZERO);
    }
    
    #[test]
    fn test_depth_3_vs_depth_2() {
        // Create a simple scene with two parallel walls
        let wall1 = Triangle {
            p0: [-10.0, -10.0, 0.0],
            p1: [10.0, -10.0, 0.0],
            p2: [10.0, 10.0, 0.0],
            n0: [0.0, 0.0, 1.0],
            n1: [0.0, 0.0, 1.0],
            n2: [0.0, 0.0, 1.0],
            color: [1.0, 0.0, 0.0], // red
        };
        
        let wall2 = Triangle {
            p0: [-10.0, -10.0, -5.0],
            p1: [10.0, 10.0, -5.0],
            p2: [10.0, -10.0, -5.0],
            n0: [0.0, 0.0, -1.0],
            n1: [0.0, 0.0, -1.0],
            n2: [0.0, 0.0, -1.0],
            color: [0.0, 1.0, 0.0], // green
        };
        
        let triangles = vec![wall1, wall2];
        let light = LightUniforms::default();
        
        let ray_origin = Vec3::new(0.0, 0.0, 5.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0);
        
        // With max_depth=2, should work fine
        let color_depth2 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 2, 0.15);
        println!("Depth 2 color: {:?}", color_depth2);
        
        // With max_depth=3, should also work (this is what we're debugging)
        let color_depth3 = trace_ray(ray_origin, ray_dir, &triangles, &light, 0, 3, 0.15);
        println!("Depth 3 color: {:?}", color_depth3);
        
        // Both should produce colored results (not black)
        assert!(color_depth2.length() > 0.1, "Depth 2 should produce color");
        assert!(color_depth3.length() > 0.1, "Depth 3 should produce color");
    }
}
