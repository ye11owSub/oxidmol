pub enum SurfaceSource {
    #[cfg(not(target_arch = "wasm32"))]
    Window(usize), // Qt winId()
    #[cfg(target_arch = "wasm32")]
    Canvas(web_sys::HtmlCanvasElement),
}

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
    pub async fn new(source: SurfaceSource, width: u32, height: u32) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = match source {
            #[cfg(not(target_arch = "wasm32"))]
            SurfaceSource::Window(handle) => {
                let window = crate::utils::QtWindowHandle::new(handle);
                let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window) }.unwrap();
                unsafe { instance.create_surface_unsafe(target) }.unwrap()
            }

            #[cfg(target_arch = "wasm32")]
            SurfaceSource::Canvas(canvas) => instance
                .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
                .unwrap(),
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
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

        Self {
            instance,
            surface,
            device,
            queue,
            config,
        }
    }
}
