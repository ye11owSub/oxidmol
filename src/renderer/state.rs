use bytemuck::cast_slice;
use glam;
use wgpu::util::DeviceExt;

use crate::{
    core::{mesh_builder, FlatAtom, Mesh, Molecule},
    renderer::{
        camera::{Camera, CameraUniform},
        camera_gpu::CameraGpu,
        depth_texture::DepthTexture,
        gpu_context::GpuContext,
        pipeline::PipelineBuilder,
    },
    utils::LoadError,
};

pub struct State<'a> {
    pub gpu: GpuContext<'a>,
    render_pipeline: wgpu::RenderPipeline,
    sphere_mesh: Mesh,
    depth: DepthTexture,
    pub camera: Camera,
    camera_gpu: CameraGpu,
    instance_buffer: Option<wgpu::Buffer>,
    instance_count: u32,
}

impl<'a> State<'a> {
    pub async fn new(window_handle: usize, width: u32, height: u32) -> Self {
        let gpu_context = GpuContext::new(window_handle, width, height).await;
        let depth_view = DepthTexture::new(&gpu_context.device, width, height);

        let camera = Camera {
            aspect: gpu_context.config.width as f32 / gpu_context.config.height as f32,
            ..Camera::default()
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_bind_group_layout =
            gpu_context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Camera BGL"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let camera_gpu = CameraGpu::new(&gpu_context.device, &camera_bind_group_layout);

        let sphere_mesh = mesh_builder::make_sphere(&gpu_context.device, 16, 16);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_buffer_layout(mesh_builder::SphereVertex::get_layout());
        pipeline_builder.add_buffer_layout(FlatAtom::get_instance_layout());
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(gpu_context.config.format);
        pipeline_builder.set_depth_format(depth_view.format());
        let render_pipeline =
            pipeline_builder.build_pipeline(&gpu_context.device, &[&camera_bind_group_layout]);

        Self {
            gpu: gpu_context,
            render_pipeline,
            sphere_mesh,
            depth: depth_view,
            camera,
            camera_gpu,
            instance_buffer: None,
            instance_count: 0,
        }
    }

    pub fn upload_molecule(&mut self, mol: &Molecule) -> Result<(), LoadError> {
        let atoms = mol.atoms_flat.clone();

        if atoms.is_empty() {
            self.instance_buffer = None;
            self.instance_count = 0;
            return Ok(());
        }

        // Bounding sphere: center + max radius
        let center = atoms
            .iter()
            .fold(glam::Vec3::ZERO, |acc, a| acc + a.position)
            / atoms.len() as f32;
        let max_r = atoms
            .iter()
            .map(|a| (a.position - center).length() + a.radius)
            .fold(0.0_f32, f32::max);

        let fov_y = self.camera.fovy.to_radians();
        let dist = max_r / (fov_y * 0.5).tan() + max_r;
        self.camera.target = center;
        self.camera.eye = center + glam::Vec3::new(0.0, 0.0, dist);
        self.camera.up = glam::Vec3::Y;
        self.camera.znear = 0.1;
        self.camera.zfar = dist * 4.0;

        let instance_buffer =
            self.gpu
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Atom instance buffer"),
                    contents: cast_slice(&atoms),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        self.instance_count = atoms.len() as u32;
        self.instance_buffer = Some(instance_buffer);

        Ok(())
    }

    pub fn load_molecule(&mut self, path: &str) -> Result<(), LoadError> {
        let mol = Molecule::load(path)?;
        self.upload_molecule(&mol)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.gpu.config.width = width;
            self.gpu.config.height = height;
            self.gpu
                .surface
                .configure(&self.gpu.device, &self.gpu.config);

            self.depth.resize(&self.gpu.device, width, height);

            self.camera.aspect = width as f32 / height as f32;
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let frame = self.gpu.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.camera_gpu.upload(&self.gpu.queue, &self.camera);

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.10,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(self.depth.attachment()),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            if let Some(inst_buf) = &self.instance_buffer {
                rp.set_pipeline(&self.render_pipeline);
                rp.set_bind_group(0, self.camera_gpu.bind_group(), &[]);
                rp.set_vertex_buffer(0, self.sphere_mesh.vertex_buffer.slice(..));
                rp.set_index_buffer(
                    self.sphere_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                rp.set_vertex_buffer(1, inst_buf.slice(..));
                rp.draw_indexed(0..self.sphere_mesh.index_count, 0, 0..self.instance_count);
            }
        }

        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}
