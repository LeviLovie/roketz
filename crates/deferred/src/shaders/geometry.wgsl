struct Object {
    pos: vec2<f32>,
    size: vec2<f32>,
    tint: u32,
    rot: f32,
    bid: u32,
    tid: u32,
};

struct Frame {
    size: vec2<u32>,
}

struct Camera {
    pos: vec2<f32>,
    size: vec2<f32>,
    z: f32,
    pad: u32,
}

struct Layer {
    z: f32,
};

// Objects
@group(0) @binding(0) var<storage, read> objects: array<Object>;
// Uniforms
@group(1) @binding(0) var<uniform> frame: Frame;
@group(1) @binding(1) var<uniform> camera: Camera;
@group(1) @binding(2) var<uniform> layer: Layer;
// Textures
@group(2) @binding(0) var textures: texture_2d_array<f32>;
@group(2) @binding(1) var tex_sampler: sampler;

var<private> quad_positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0)
);

struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) tint: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) tid: u32,
};

fn rotate(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return vec2<f32>(
        p.x * c - p.y * s,
        p.x * s + p.y * c
    );
}

fn unpack_color(c: u32) -> vec4<f32> {
    let r: f32 = f32((c >> 16u) & 0xFFu) / 255.0;
    let g: f32 = f32((c >> 8u)  & 0xFFu) / 255.0;
    let b: f32 = f32(c & 0xFFu) / 255.0;
    let a: f32 = f32((c >> 24u) & 0xFFu) / 255.0;
    return vec4<f32>(r, g, b, a);
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32,
           @builtin(instance_index) ii: u32) -> VSOut {
    let quad = quad_positions[vi];
    let obj = objects[ii];

    // Main layer is at z=0.0, yes this is hard coded.
    // U can add it to the camera uniform if u really want to.
    // Btw that is going to get rid of the padding :)
    let parallax_offset = layer.z / (layer.z + camera.z) * camera.pos;

    let local = (quad - vec2<f32>(0.5, 0.5)) * obj.size;
    let rotated = rotate(local, obj.rot);
    let pixel_pos = obj.pos - camera.pos - parallax_offset + rotated;

    let norm_x = pixel_pos.x / camera.size.x;
    let norm_y = pixel_pos.y / camera.size.y;

    let ndc_x = norm_x * 2.0 - 1.0;
    let ndc_y = 1.0 - norm_y * 2.0;

    var out: VSOut;
    out.pos = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.tint = unpack_color(obj.tint);
    out.tex_coords = quad;
    out.tid = obj.tid;

    return out;
}

@fragment
fn fs_main(input: VSOut) -> @location(0) vec4<f32> {
    let color = textureSample(textures, tex_sampler, input.tex_coords, input.tid);
    return color * input.tint;
}
