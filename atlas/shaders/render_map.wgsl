@group(0) @binding(0) var time: f32;
@group(0) @binding(1) var map_zoom: f32;                // zoom of the map
@group(0) @binding(2) var map_translation: vec2u;       // the movement of the map in pixels
@group(0) @binding(3) var window_size: vec2u;           // window size in pixels

@group(1) @binding(0) var map_sampler: sampler;         // the sampler of the map
@group(1) @binding(1) var map_texture: texture_2d<f32>; // the map texture

@fragment
fn fragmentMain(@location(0) uv: vec2f) -> @location(0) vec4f {
    map_dims = textureDimensions(map_texture, 0);
    vec2f map_uv = (uv*window_size + map_translation) / map_dims / map_zoom;

    return vec4f(textureSample(map_texture, map_sampler, map_uv), 1.);
}

@vertex
fn vertexMain(@location(0) xyz: vec3f) -> @location(0) vec3f{
    return xyz;
}