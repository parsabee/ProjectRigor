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

kernel void raytrace_kernel(
    texture2d<float, access::write> output_texture [[texture(0)]],
    constant Triangle* triangles [[buffer(0)]],
    constant RayTracingParams& params [[buffer(1)]],
    constant LightUniforms& light [[buffer(2)]],
    uint2 gid [[thread_position_in_grid]]
) {
    uint width = output_texture.get_width();
    uint height = output_texture.get_height();
    
    if (gid.x >= width || gid.y >= height) {
        return;
    }
    
    // Generate ray from camera through pixel
    float u = (float(gid.x) + 0.5) / float(width);
    float v = (float(gid.y) + 0.5) / float(height);
    
    // Convert to NDC space [-1, 1]
    float ndc_x = u * 2.0 - 1.0;
    float ndc_y = 1.0 - v * 2.0; // Flip Y
    
    // Compute camera basis vectors from position and target
    float3 camera_forward = normalize(float3(params.camera_target) - float3(params.camera_position));
    float3 world_up = float3(0.0, 1.0, 0.0);
    float3 camera_right = normalize(cross(camera_forward, world_up));
    float3 camera_up = cross(camera_right, camera_forward);
    
    // Calculate ray direction in world space directly
    float fov = M_PI_F / 4.0; // 45 degrees
    float half_height = tan(fov / 2.0);
    float half_width = params.aspect_ratio * half_height;
    
    float3 ray_dir = normalize(
        camera_forward +
        camera_right * (ndc_x * half_width) +
        camera_up * (ndc_y * half_height)
    );
    
    // Trace ray
    float closest_t = INFINITY;
    int closest_idx = -1;
    float closest_u = 0.0;
    float closest_v = 0.0;
    
    for (uint i = 0; i < uint(params.triangle_count); i++) {
        float t, tri_u, tri_v;
        if (ray_triangle_intersection(float3(params.camera_position), ray_dir, triangles[i], t, tri_u, tri_v)) {
            if (t < closest_t) {
                closest_t = t;
                closest_idx = i;
                closest_u = tri_u;
                closest_v = tri_v;
            }
        }
    }
    
    float3 color;
    if (closest_idx >= 0) {
        Triangle tri = triangles[closest_idx];
        float w = 1.0 - closest_u - closest_v;
        
        // Interpolate normal
        float3 normal = normalize(w * tri.n0 + closest_u * tri.n1 + closest_v * tri.n2);
        
        // Calculate hit point
        float3 hit_point = float3(params.camera_position) + ray_dir * closest_t;
        
        // View direction (from hit point to camera)
        float3 view_dir = normalize(float3(params.camera_position) - hit_point);
        
        // Phong lighting with configurable parameters
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
        color = tri.color * lighting;
    } else {
        // Background color
        color = float3(0.1, 0.1, 0.15);
    }
    
    output_texture.write(float4(color, 1.0), gid);
}
