use glam::{Mat4, Quat, Vec3};

pub fn create_view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    Mat4::look_at_rh(eye, target, up)
}

pub fn create_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh(fov, aspect_ratio, near, far)
}

pub fn create_transform_matrix(translation: Vec3, rotation: Quat, scale: Vec3) -> Mat4 {
    Mat4::from_translation(translation) * Mat4::from_quat(rotation) * Mat4::from_scale(scale)
}
