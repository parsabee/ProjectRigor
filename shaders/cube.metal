#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float3 position [[attribute(0)]];
    float3 normal [[attribute(1)]];
    float3 color [[attribute(2)]];
};

struct VertexOut {
    float4 position [[position]];
    float3 worldNormal;
    float3 worldPosition;
    float3 color;
};

struct Uniforms {
    float4x4 modelViewProjection;
};

struct Light {
    float3 direction;
    float3 color;
    float ambientIntensity;
    float diffuseIntensity;
    float specularIntensity;
    float shininess;
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]],
                             constant Uniforms& uniforms [[buffer(1)]]) {
    VertexOut out;
    out.position = uniforms.modelViewProjection * float4(in.position, 1.0);
    out.worldNormal = in.normal; // For now, assuming no rotation (will fix with normal matrix later)
    out.worldPosition = in.position;
    out.color = in.color;
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]],
                              constant Light& light [[buffer(0)]]) {
    // Normalize the interpolated normal
    float3 normal = normalize(in.worldNormal);
    
    // Normalize light direction
    float3 lightDir = normalize(-light.direction);
    
    // Ambient component
    float3 ambient = light.ambientIntensity * light.color;
    
    // Diffuse component (Lambertian reflection)
    float diff = max(dot(normal, lightDir), 0.0);
    float3 diffuse = light.diffuseIntensity * diff * light.color;
    
    // Specular component (Blinn-Phong)
    float3 viewDir = normalize(float3(0.0, 0.0, 1.0)); // Simplified camera direction
    float3 halfDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(normal, halfDir), 0.0), light.shininess);
    float3 specular = light.specularIntensity * spec * light.color;
    
    // Combine lighting with surface color
    float3 result = (ambient + diffuse + specular) * in.color;
    
    return float4(result, 1.0);
}
