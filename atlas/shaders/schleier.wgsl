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

@fragment
fn fragmentMain(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4f(0.);
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