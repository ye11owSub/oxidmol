struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) local_pos: vec3<f32>,
};

struct InstanceInput {
    @location(1) center: vec3<f32>,
    @location(2) radius: f32,
    @location(3) color:  vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos:     vec4<f32>,
    @location(0)       world_normal: vec3<f32>,
    @location(1)       color:        vec3<f32>,
};

@vertex
fn vs_main(v: VertexInput, inst: InstanceInput) -> VertexOutput {
    let world_pos = inst.center + v.local_pos * inst.radius;
    var out: VertexOutput;
    out.clip_pos     = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.world_normal = v.local_pos; // unit sphere: outward normal == local position
    out.color        = inst.color;
    return out;
}

// Normalized (1,1,1) direction
const LIGHT: vec3<f32> = vec3<f32>(0.5774, 0.5774, 0.5774);

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let n       = normalize(in.world_normal);
    let diffuse = max(dot(n, LIGHT), 0.0);
    let lit     = in.color * (0.15 + diffuse * 0.85);
    return vec4<f32>(lit, 1.0);
}
