use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

pub struct Camera {
    pub(crate) eye: Vec3,
    pub(crate) target: Vec3,
    pub(crate) up: Vec3,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array_2d(&[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.5, 1.0],
]);

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::Y,
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

impl Camera {
    fn build_view_pojection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn reset(&mut self) {
        let d = Camera::default();
        self.eye = d.eye;
        self.target = d.target;
        self.up = d.up;
        self.fovy = d.fovy;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.target - self.eye).normalize();
        let distance = (self.target - self.eye).length();

        let new_distance = (distance * 0.9_f32.powf(delta)).clamp(0.1, 1000.0);

        self.eye = self.target - direction * new_distance;
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.005;

        let offset = self.eye - self.target;
        let distance = offset.length();

        let yaw = offset.z.atan2(offset.x);
        let pitch = (offset.y / distance).asin();

        let new_yaw = yaw + dx * sensitivity;
        let new_pitch = (pitch - dy * sensitivity).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );

        self.eye = self.target
            + Vec3::new(
                distance * new_pitch.cos() * new_yaw.cos(),
                distance * new_pitch.sin(),
                distance * new_pitch.cos() * new_yaw.sin(),
            );
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.002;

        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(self.up).normalize();
        let up = right.cross(forward).normalize();

        let distance = (self.target - self.eye).length();
        let offset = (-right * dx + up * dy) * sensitivity * distance;

        self.eye += offset;
        self.target += offset;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub(crate) struct CameraUniform {
    pub(crate) view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_pojection_matrix().to_cols_array_2d();
    }
}
