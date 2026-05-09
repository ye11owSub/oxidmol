use bytemuck::{bytes_of, cast_slice};
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

use crate::{
    core::{mesh_builder, molecule::FlatAtom, Mesh, Molecule},
    renderer::camera::{make_projection, CameraUniform},
    renderer::pipeline::PipelineBuilder,
    utils::{LoadError, QtWindowHandle},
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub struct State<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    #[allow(dead_code)]
    config: wgpu::SurfaceConfiguration,
    pub size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline,
    sphere_mesh: Mesh,
    depth_view: wgpu::TextureView,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instance_buffer: Option<wgpu::Buffer>,
    instance_count: u32,
}

impl<'a> State<'a> {
    pub async fn new(window_handle: usize, width: u32, height: u32) -> Self {
        let size = (width, height);
        let window = QtWindowHandle::new(window_handle);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window) }.unwrap();
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                memory_hints: wgpu::MemoryHints::default(),
                required_limits: wgpu::Limits::default(),
                trace: wgpu::Trace::Off,
                label: None,
            })
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Depth texture
        let depth_view = Self::make_depth_view(&device, width, height);

        // Camera uniform buffer (identity as placeholder)
        let camera_data = CameraUniform {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytes_of(&camera_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &camera_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Sphere mesh (16×16 stacks/slices — smooth enough, cheap enough)
        let sphere_mesh = mesh_builder::make_sphere(&device, 16, 16);

        // Render pipeline
        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_buffer_layout(mesh_builder::SphereVertex::get_layout());
        pipeline_builder.add_buffer_layout(FlatAtom::get_instance_layout());
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        pipeline_builder.set_depth_format(DEPTH_FORMAT);
        let render_pipeline = pipeline_builder.build_pipeline(&device, &[&camera_bgl]);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            sphere_mesh,
            depth_view,
            camera_buffer,
            camera_bind_group,
            instance_buffer: None,
            instance_count: 0,
        }
    }

    fn make_depth_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Upload an already-parsed molecule to the GPU.
    pub fn upload_molecule(&mut self, mol: &Molecule) -> Result<(), LoadError> {
        let atoms = mol.atoms_flat.clone();

        if atoms.is_empty() {
            self.instance_buffer = None;
            self.instance_count = 0;
            return Ok(());
        }

        // Bounding sphere: center + max radius
        let center = atoms.iter().fold(Vec3::ZERO, |acc, a| acc + a.position) / atoms.len() as f32;
        let max_r = atoms
            .iter()
            .map(|a| (a.position - center).length() + a.radius)
            .fold(0.0_f32, f32::max);

        // Position camera so the molecule fits in the 45° FOV
        let fov_y: f32 = 45.0_f32.to_radians();
        let dist = max_r / (fov_y * 0.5).tan();
        let eye = center + Vec3::new(0.0, 0.0, dist + max_r);
        let far = (dist + max_r) * 4.0;

        let view = Mat4::look_at_rh(eye, center, Vec3::Y);
        let aspect = self.size.0 as f32 / self.size.1 as f32;
        let proj = make_projection(fov_y, aspect, 0.1, far);
        let view_proj = proj * view;

        let camera_data = CameraUniform {
            view_proj: view_proj.to_cols_array_2d(),
        };
        self.queue
            .write_buffer(&self.camera_buffer, 0, bytes_of(&camera_data));

        // Upload atom instance data
        let instance_buffer = self
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

    /// Load a PDB or mmCIF file and upload atom data to the GPU.
    pub fn load_molecule(&mut self, path: &str) -> Result<(), LoadError> {
        let mol = Molecule::load(path)?;
        self.upload_molecule(&mol)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);

            self.depth_view = Self::make_depth_view(&self.device, width, height);

            let aspect = self.config.width as f32 / self.config.height as f32;

            let projection = glam::Mat4::perspective_lh(45.0_f32.to_radians(), aspect, 0.1, 100.0);

            let view = glam::Mat4::look_at_lh(
                glam::Vec3::new(0.0, 0.0, -5.0),
                glam::Vec3::ZERO,
                glam::Vec3::Y,
            );

            let view_proj = projection * view;

            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytes_of(&view_proj.to_cols_array_2d()),
            );
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            if let Some(inst_buf) = &self.instance_buffer {
                rp.set_pipeline(&self.render_pipeline);
                rp.set_bind_group(0, &self.camera_bind_group, &[]);
                rp.set_vertex_buffer(0, self.sphere_mesh.vertex_buffer.slice(..));
                rp.set_index_buffer(
                    self.sphere_mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                rp.set_vertex_buffer(1, inst_buf.slice(..));
                rp.draw_indexed(0..self.sphere_mesh.index_count, 0, 0..self.instance_count);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}
