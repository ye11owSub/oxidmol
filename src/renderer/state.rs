use crate::{
    core::Molecule,
    renderer::{gpu_context::GpuContext, scene::Scene},
    utils::{error::WgpuError, LoadError},
};

/// A scene bound to a window surface.
pub struct State<'a> {
    pub gpu: GpuContext<'a>,
    pub scene: Scene,
}

impl<'a> State<'a> {
    pub async fn new(window_handle: usize, width: u32, height: u32) -> Result<Self, WgpuError> {
        let gpu = GpuContext::new(window_handle, width, height).await?;
        let scene = Scene::new(
            &gpu.device,
            gpu.config.format,
            gpu.config.width,
            gpu.config.height,
        );

        Ok(Self { gpu, scene })
    }

    pub fn upload_molecule(&mut self, mol: &Molecule) -> Result<(), LoadError> {
        self.scene.add_molecule(&self.gpu.device, mol);
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
            self.scene.resize(&self.gpu.device, width, height);
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        let frame = match self.gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.gpu
                    .surface
                    .configure(&self.gpu.device, &self.gpu.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("surface validation error".into());
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.scene.render(&self.gpu.device, &self.gpu.queue, &view);
        self.gpu.queue.present(frame);
        Ok(())
    }
}
