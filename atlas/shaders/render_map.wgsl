struct Metadata{
    time: f32,
    map_zoom: f32,
    map_translation: vec2f,       // the movement of the map in pixels
    window_size: vec2f,           // window size in pixels
    mouse_pos: vec2f,
}
struct VertexOut{
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
}

@group(0) @binding(0) var<uniform> metadata: Metadata;

@group(1) @binding(0) var map_sampler: sampler;         // the sampler of the map
@group(1) @binding(1) var map_texture: texture_2d<f32>; // the map texture

@fragment
fn fragmentMain(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // the map is (original_size * zoom) pixels wide/high
    var map_dims = vec2f(textureDimensions(map_texture, 0));
    var map_uv = vec2f(uv.x, 1.-uv.y);
    map_uv = map_uv + (metadata.map_translation / metadata.window_size);

    // zoom cetered at the WINDOW center (not the map center)
    map_uv = (map_uv-vec2f(0.5))*metadata.map_zoom+vec2f(0.5);

    // scale to map dimensions
    map_uv = map_uv * (metadata.window_size / map_dims);

    // return vec4f(map_uv, 0.,1.);
    return vec4f(textureSample(map_texture, map_sampler, map_uv).rgb, 1.);
}

@vertex
fn vertexMain(@builtin(vertex_index) vidx: u32) -> VertexOut {
    const pos = array<vec2<f32>, 3> (
        vec2f(-1, 3),
        vec2f(-1, -1),
        vec2f(3, -1),
    );
    var out: VertexOut;

    out.position = vec4f(pos[vidx], 0., 1.);
    out.uv = (pos[vidx] + vec2f(1, 1.))/2.;

    return out;
}