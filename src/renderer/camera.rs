use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub(crate) view_proj: [[f32; 4]; 4],
}

/// Builds a perspective matrix compatible with wgpu's NDC (depth [0, 1]).
pub(crate) fn make_projection(fov_y: f32, aspect: f32, near: f32, far: f32) -> glam::Mat4 {
    // create_projection_matrix uses glam's perspective_rh (depth [-1, 1]).
    // Apply correction to remap Z from [-1, 1] → [0, 1] as wgpu expects.
    let proj = Mat4::perspective_rh(fov_y, aspect, near, far);
    let correction = glam::Mat4::from_cols_array_2d(&[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5, 1.0],
    ]);
    correction * proj
}
