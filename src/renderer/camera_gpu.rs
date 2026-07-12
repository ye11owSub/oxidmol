use crate::renderer::camera::{Camera, CameraUniform};
use wgpu::util::DeviceExt;

pub struct CameraGpu {
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl CameraGpu {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::bytes_of(&CameraUniform::new()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self { buffer, bind_group }
    }

    pub fn upload(&self, queue: &wgpu::Queue, camera: &Camera) {
        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(camera);
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
