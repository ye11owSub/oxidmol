use crate::utils::{error::WgpuError, QtWindowHandle};

pub struct GpuContext<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    #[allow(dead_code)]
    pub config: wgpu::SurfaceConfiguration,
}

impl<'a> GpuContext<'a> {
    pub async fn new(window_handle: usize, width: u32, height: u32) -> Result<Self, WgpuError> {
        let window = QtWindowHandle::new(window_handle)?;

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..wgpu::InstanceDescriptor::new_with_display_handle(Box::new(window.display()))
        });

        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window) }
            .map_err(|e| WgpuError::SurfaceCreation(e.to_string()))?;
        let surface = unsafe { instance.create_surface_unsafe(target) }
            .map_err(|e| WgpuError::SurfaceCreation(e.to_string()))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .map_err(|_| WgpuError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                memory_hints: wgpu::MemoryHints::default(),
                required_limits: wgpu::Limits::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                label: None,
            })
            .await
            .map_err(|e| WgpuError::DeviceCreation(e.to_string()))?;

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| caps.formats.first().copied())
            .ok_or_else(|| WgpuError::SurfaceCreation("surface supports no formats".into()))?;
        let present_mode = caps.present_modes.first().copied().ok_or_else(|| {
            WgpuError::SurfaceCreation("surface supports no present modes".into())
        })?;
        let alpha_mode =
            caps.alpha_modes.first().copied().ok_or_else(|| {
                WgpuError::SurfaceCreation("surface supports no alpha modes".into())
            })?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // a widget can be created before its first layout pass, wgpu rejects zero-sized surfaces
            width: width.max(1),
            height: height.max(1),
            present_mode,
            alpha_mode,
            view_formats: vec![],
            color_space: wgpu::SurfaceColorSpace::Auto,
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Ok(Self {
            instance,
            surface,
            device,
            queue,
            config,
        })
    }
}
