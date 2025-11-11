// Copyright (c) 2025 Parsa Bagheri
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#include <metal_stdlib>
using namespace metal;

struct Triangle {
    packed_float3 p0;
    packed_float3 p1;
    packed_float3 p2;
    packed_float3 n0;
    packed_float3 n1;
    packed_float3 n2;
    packed_float3 color;
};

struct RayTracingParams {
    packed_float3 camera_position;
    float aspect_ratio;
    packed_float3 camera_target;
    float triangle_count;
};

struct LightUniforms {
    packed_float3 direction;
    float _padding1;
    packed_float3 color;
    float _padding2;
    float ambient_intensity;
    float diffuse_intensity;
    float specular_intensity;
    float shininess;
};

// Möller–Trumbore ray-triangle intersection
bool ray_triangle_intersection(
    float3 ray_origin,
    float3 ray_dir,
    Triangle tri,
    thread float& t,
    thread float& u,
    thread float& v
) {
    const float EPSILON = 0.000001;
    
    float3 edge1 = tri.p1 - tri.p0;
    float3 edge2 = tri.p2 - tri.p0;
    
    float3 h = cross(ray_dir, edge2);
    float a = dot(edge1, h);
    
    if (abs(a) < EPSILON) {
        return false; // Ray is parallel
    }
    
    float f = 1.0 / a;
    float3 s = ray_origin - tri.p0;
    u = f * dot(s, h);
    
    if (u < 0.0 || u > 1.0) {
        return false;
    }
    
    float3 q = cross(s, edge1);
    v = f * dot(ray_dir, q);
    
    if (v < 0.0 || u + v > 1.0) {
        return false;
    }
    
    t = f * dot(edge2, q);
    
    return t > EPSILON;
}

// Trace a ray and return the color with reflections
float3 trace_ray(
    float3 ray_origin,
    float3 ray_dir,
    constant Triangle* triangles,
    uint triangle_count,
    constant LightUniforms& light,
    int depth
) {
    const int MAX_DEPTH = 2;
    const float EPSILON = 0.001;
    const float REFLECTIVITY = 0.15;
    
    if (depth >= MAX_DEPTH) {
        return float3(0.0);
    }
    
    // Find closest intersection
    float closest_t = INFINITY;
    int closest_idx = -1;
    float closest_u = 0.0;
    float closest_v = 0.0;
    
    for (uint i = 0; i < triangle_count; i++) {
        float t, tri_u, tri_v;
        if (ray_triangle_intersection(ray_origin, ray_dir, triangles[i], t, tri_u, tri_v)) {
            if (t < closest_t) {
                closest_t = t;
                closest_idx = i;
                closest_u = tri_u;
                closest_v = tri_v;
            }
        }
    }
    
    if (closest_idx < 0) {
        // No intersection - return background color
        return float3(0.2, 0.0, 0.2);  // dark purple
    }
    
    Triangle tri = triangles[closest_idx];
    float w = 1.0 - closest_u - closest_v;
    
    // Interpolate normal
    float3 normal = normalize(w * tri.n0 + closest_u * tri.n1 + closest_v * tri.n2);
    
    // Calculate hit point
    float3 hit_point = ray_origin + ray_dir * closest_t;
    
    // View direction (from hit point to camera)
    float3 view_dir = -ray_dir;
    
    // Phong lighting
    float3 light_dir_normalized = normalize(light.direction);
    float ndotl = max(dot(normal, -light_dir_normalized), 0.0);
    
    // Ambient component
    float3 ambient = light.ambient_intensity * light.color;
    
    // Diffuse component
    float3 diffuse = light.diffuse_intensity * ndotl * light.color;
    
    // Specular component (Blinn-Phong)
    float3 specular = float3(0.0);
    if (ndotl > 0.0) {
        float3 half_dir = normalize(-light_dir_normalized + view_dir);
        float spec = pow(max(dot(normal, half_dir), 0.0), light.shininess);
        specular = light.specular_intensity * spec * light.color;
    }
    
    // Combine lighting components
    float3 lighting = ambient + diffuse + specular;
    float3 local_color = tri.color * lighting;
    
    // Trace reflections (if not at max depth)
    if (depth < MAX_DEPTH - 1) {
        float3 reflection_dir = reflect(ray_dir, normal);
        float3 reflection_origin = hit_point + normal * EPSILON;
        float3 reflection_color = trace_ray(reflection_origin, reflection_dir, triangles, triangle_count, light, depth + 1);
        
        // Blend local color with reflection
        return mix(local_color, reflection_color, REFLECTIVITY);
    }
    
    return local_color;
}

kernel void raytrace_kernel(
    texture2d<float, access::write> output_texture [[texture(0)]],
    constant Triangle* triangles [[buffer(0)]],
    constant RayTracingParams& params [[buffer(1)]],
    constant LightUniforms& light [[buffer(2)]],
    constant uint& triangle_count [[buffer(3)]],
    uint2 gid [[thread_position_in_grid]]
) {
    // Get texture dimensions
    uint width = output_texture.get_width();
    uint height = output_texture.get_height();
    
    // Check bounds
    if (gid.x >= width || gid.y >= height) {
        return;
    }
    
    // Calculate camera basis
    float3 forward = normalize(params.camera_target - params.camera_position);
    float3 right = normalize(cross(forward, float3(0, 1, 0)));
    float3 up = cross(right, forward);
    
    // NDC coordinates (-1 to 1)
    float aspect = float(width) / float(height);
    float2 ndc = float2(
        (float(gid.x) / float(width) - 0.5) * 2.0,
        (0.5 - float(gid.y) / float(height)) * 2.0
    );
    
    float fov = 60.0 * 3.14159265 / 180.0;
    float half_height = tan(fov / 2.0);
    
    // Calculate ray direction
    float3 ray_dir = normalize(
        forward + 
        right * ndc.x * half_height * aspect +
        up * ndc.y * half_height
    );
    
    // Trace the ray with reflections using the trace_ray function
    float3 color = trace_ray(params.camera_position, ray_dir, triangles, triangle_count, light, 0);
    
    // Write to output texture (BGRA format)
    output_texture.write(float4(color, 1.0), gid);
}
