#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct QuadInstance {
    packed_float2 position;
    packed_float2 size;
    packed_float4 color;
};

struct QuadVertexOutput {
    float4 position [[position]];
    float4 color;
};

vertex QuadVertexOutput quad_vertex(
    uint vertex_id [[vertex_id]],
    uint instance_id [[instance_id]],
    const device float2* vertices [[buffer(0)]],
    const device QuadInstance* quads [[buffer(1)]],
    const device float2* viewport_size [[buffer(2)]] 
) {
    QuadVertexOutput output;
    
    // Get the current quad
    QuadInstance quad = quads[instance_id];
    
    // Get the vertex position from the vertex buffer
    float2 vertex_pos = vertices[vertex_id];
    
    // Calculate the final position
    float2 pos = float2(quad.position) + (vertex_pos * float2(quad.size));
    
    // Convert to normalized device coordinates (-1 to 1)
    float2 normalized_pos = (pos / float2(*viewport_size)) * 2.0 - 1.0;
    // Flip Y coordinate for Metal's coordinate system
    normalized_pos.y = -normalized_pos.y;
    
    output.position = float4(normalized_pos, 0.0, 1.0);
    output.color = float4(quad.color);
    
    return output;
}

fragment float4 quad_fragment(
    QuadVertexOutput in [[stage_in]]
) {
    return in.color;
}

