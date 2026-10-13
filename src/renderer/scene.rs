use crate::{
    core::Molecule,
    renderer::{
        camera::Camera, camera_gpu::CameraGpu, depth_texture::DepthTexture,
        scene_object::SceneObject, scene_pipeline::ScenePipeline,
    },
};

const BACKGROUND: wgpu::Color = wgpu::Color {
    r: 0.05,
    g: 0.05,
    b: 0.10,
    a: 1.0,
};

/// Everything that is drawn, independent of where it is drawn to (a window surface or a texture).
pub struct Scene {
    pipeline: ScenePipeline,
    depth: DepthTexture,
    pub camera: Camera,
    camera_gpu: CameraGpu,
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let camera = Camera {
            aspect: width as f32 / height as f32,
            ..Camera::default()
        };

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        Self {
            camera_gpu: CameraGpu::new(device, &camera_bind_group_layout),
            pipeline: ScenePipeline::new(device, target_format, &camera_bind_group_layout),
            depth: DepthTexture::new(device, width, height),
            camera,
            objects: Vec::new(),
        }
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Adds the molecule to the scene and points the camera at it.
    pub fn add_molecule(&mut self, device: &wgpu::Device, mol: &Molecule) {
        let atoms = &mol.atoms_flat;
        if atoms.is_empty() {
            return;
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

        self.objects.push(SceneObject::from_molecule(
            device,
            "Atom instance buffer".to_string(),
            mol,
        ));
    }

    /// Call whenever the render target changes size.
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.depth.resize(device, width, height);
            self.camera.aspect = width as f32 / height as f32;
        }
    }

    /// Draws the scene into `target` and submits the work to the queue.
    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue, target: &wgpu::TextureView) {
        self.camera_gpu.upload(queue, &self.camera);

        let mut encoder = device.create_command_encoder(&Default::default());

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(self.depth.attachment()),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            rp.set_pipeline(&self.pipeline.render_pipeline);
            rp.set_bind_group(0, &self.camera_gpu.bind_group, &[]);
            rp.set_vertex_buffer(0, self.pipeline.sphere_mesh.vertex_buffer.slice(..));
            rp.set_index_buffer(
                self.pipeline.sphere_mesh.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            for object in &self.objects {
                object.draw(&mut rp, &self.pipeline.sphere_mesh);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}

/// Tests that run the real pipeline and shader on a GPU, rendering to a texture instead of a window.
#[cfg(test)]
mod gpu_tests {
    use super::*;
    use crate::renderer::test_support::{
        assert_no_validation_error, gpu, read_image, target_texture, Image, FORMAT, HEIGHT, WIDTH,
    };

    // BACKGROUND (0.05, 0.05, 0.10) as Rgba8 bytes; +-1 for rounding differences between GPUs
    fn is_background(px: [u8; 4]) -> bool {
        let near = |a: u8, b: u8| a.abs_diff(b) <= 1;
        near(px[0], 13) && near(px[1], 13) && near(px[2], 26) && px[3] == 255
    }

    fn render(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        width: u32,
        height: u32,
    ) -> Image {
        let texture = target_texture(device, width, height);
        let view = texture.create_view(&Default::default());
        assert_no_validation_error(device, || scene.render(device, queue, &view));
        read_image(device, queue, &texture, width, height)
    }

    fn scene_with_molecule(device: &wgpu::Device) -> Scene {
        let mut scene = Scene::new(device, FORMAT, WIDTH, HEIGHT);
        let mol = Molecule::load("tests/data/minimal.pdb").expect("test molecule");
        scene.add_molecule(device, &mol);
        scene
    }

    #[test]
    fn test_shader_and_pipeline_are_valid() {
        let Some((device, _queue)) = gpu() else {
            return;
        };

        assert_no_validation_error(&device, || Scene::new(&device, FORMAT, WIDTH, HEIGHT));
    }

    #[test]
    fn test_empty_scene_is_all_background() {
        let Some((device, queue)) = gpu() else { return };
        let scene = Scene::new(&device, FORMAT, WIDTH, HEIGHT);

        let image = render(&device, &queue, &scene, WIDTH, HEIGHT);

        assert_eq!(image.count_where(|px| !is_background(px)), 0);
    }

    #[test]
    fn test_molecule_is_drawn_in_the_middle_not_at_the_edges() {
        let Some((device, queue)) = gpu() else { return };
        let scene = scene_with_molecule(&device);

        let image = render(&device, &queue, &scene, WIDTH, HEIGHT);

        assert!(
            !is_background(image.pixel(WIDTH / 2, HEIGHT / 2)),
            "center is empty"
        );
        for corner in [
            (0, 0),
            (WIDTH - 1, 0),
            (0, HEIGHT - 1),
            (WIDTH - 1, HEIGHT - 1),
        ] {
            assert!(
                is_background(image.pixel(corner.0, corner.1)),
                "corner {corner:?} is drawn"
            );
        }
    }

    #[test]
    fn test_zooming_out_shrinks_the_molecule() {
        let Some((device, queue)) = gpu() else { return };
        let mut scene = scene_with_molecule(&device);

        let covered = |image: &Image| image.count_where(|px| !is_background(px));
        let near = covered(&render(&device, &queue, &scene, WIDTH, HEIGHT));
        scene.camera.zoom(-5.0);
        let far = covered(&render(&device, &queue, &scene, WIDTH, HEIGHT));

        assert!(near > 0, "nothing drawn at all");
        assert!(
            far < near,
            "zooming out must cover fewer pixels ({far} !< {near})"
        );
    }

    #[test]
    fn test_resize_to_another_aspect_ratio_keeps_rendering() {
        let Some((device, queue)) = gpu() else { return };
        let mut scene = scene_with_molecule(&device);

        let (width, height) = (96, 48);
        assert_no_validation_error(&device, || scene.resize(&device, width, height));
        let image = render(&device, &queue, &scene, width, height);

        assert!((scene.camera.aspect - 2.0).abs() < 1e-6);
        assert!(
            !is_background(image.pixel(width / 2, height / 2)),
            "center is empty"
        );
    }

    #[test]
    fn test_zero_size_resize_is_ignored() {
        let Some((device, _queue)) = gpu() else {
            return;
        };
        let mut scene = Scene::new(&device, FORMAT, WIDTH, HEIGHT);
        let aspect = scene.camera.aspect;

        scene.resize(&device, 0, 0);

        assert_eq!(scene.camera.aspect, aspect);
    }

    #[test]
    fn test_empty_molecule_leaves_the_scene_unchanged() {
        let Some((device, _queue)) = gpu() else {
            return;
        };
        let mut scene = Scene::new(&device, FORMAT, WIDTH, HEIGHT);
        let empty = Molecule::new(None, Vec::new(), Vec::new());

        scene.add_molecule(&device, &empty);

        assert_eq!(scene.object_count(), 0);
    }
}
