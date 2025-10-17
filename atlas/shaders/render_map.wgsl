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

fn random2(st: vec2f) -> vec2f{
    var out = vec2( dot(st,vec2f(127.1,311.7)),
              dot(st,vec2f(269.5,183.3)) );
    return -1.0 + 2.0*fract(sin(out)*43758.5453123);
}

fn random1(x: f32) -> f32{
    return random2(vec2f(0., x)).x;
}
fn noise(st: vec2f) -> f32 {
    var i = floor(st);
    var f = fract(st);

    var u = f*f*(3.0-2.0*f);

    return mix( mix( dot( random2(i + vec2(0.0,0.0) ), f - vec2(0.0,0.0) ),
                     dot( random2(i + vec2(1.0,0.0) ), f - vec2(1.0,0.0) ), u.x),
                mix( dot( random2(i + vec2(0.0,1.0) ), f - vec2(0.0,1.0) ),
                     dot( random2(i + vec2(1.0,1.0) ), f - vec2(1.0,1.0) ), u.x), u.y);
}

fn schleier(uv: vec2f) -> vec3f {
    var mod_time = metadata.time / 4.;
    var unit_circle = vec2f(sin(mod_time), cos(mod_time))/2. + vec2f(0.5);
    var movement_noise = noise(unit_circle);
    var mod_uv = movement_noise + uv;

    var noise_value = vec3f(noise(mod_uv*10.));
    return noise_value;
}

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
    var result = textureSample(map_texture, map_sampler, map_uv).rgb;
    return vec4f(mix(result, schleier(uv), 0.2),1.);
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