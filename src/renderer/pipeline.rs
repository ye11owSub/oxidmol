pub struct PipelineBuilder {
    shader_source: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineBuilder {
    pub fn new() -> Self {
        PipelineBuilder {
            shader_source: String::new(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            depth_format: None,
            vertex_buffer_layouts: Vec::new(),
        }
    }

    /// `shader_source` is WGSL text, normally embedded with `include_str!` so that it ships inside
    /// the binary and does not depend on the working directory.
    pub fn set_shader_module(
        &mut self,
        shader_source: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_source = shader_source.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn set_depth_format(&mut self, format: wgpu::TextureFormat) {
        self.depth_format = Some(format);
    }

    pub fn add_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout)
    }

    pub fn build_pipeline(
        &mut self,
        device: &wgpu::Device,
        bind_group_layouts: &[Option<&wgpu::BindGroupLayout>],
    ) -> wgpu::RenderPipeline {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(self.shader_source.as_str().into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts,
            immediate_size: 0,
        });

        let vertex_buffers: Vec<Option<wgpu::VertexBufferLayout>> = self
            .vertex_buffer_layouts
            .iter()
            .cloned()
            .map(Some)
            .collect();

        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some(&self.vertex_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &vertex_buffers,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &render_targets,
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            depth_stencil: self.depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),

            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        };

        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_builder_new() {
        let builder = PipelineBuilder::new();

        assert!(builder.shader_source.is_empty());
        assert_eq!(builder.vertex_entry, "dummy");
        assert_eq!(builder.fragment_entry, "dummy");
        assert_eq!(builder.pixel_format, wgpu::TextureFormat::Rgba8Unorm);
        assert!(builder.vertex_buffer_layouts.is_empty());
    }

    #[test]
    fn test_set_shader_module() {
        let mut builder = PipelineBuilder::new();

        builder.set_shader_module("// wgsl", "vs_main", "fs_main");

        assert_eq!(builder.shader_source, "// wgsl");
        assert_eq!(builder.vertex_entry, "vs_main");
        assert_eq!(builder.fragment_entry, "fs_main");
    }

    #[test]
    fn test_set_pixel_format() {
        let mut builder = PipelineBuilder::new();

        builder.set_pixel_format(wgpu::TextureFormat::Bgra8Unorm);

        assert_eq!(builder.pixel_format, wgpu::TextureFormat::Bgra8Unorm);
    }

    #[test]
    fn test_add_buffer_layout() {
        let mut builder = PipelineBuilder::new();

        let layout = wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[],
        };

        builder.add_buffer_layout(layout);

        assert_eq!(builder.vertex_buffer_layouts.len(), 1);
        assert_eq!(builder.vertex_buffer_layouts[0].array_stride, 12);
    }

    #[test]
    fn test_add_multiple_buffer_layouts() {
        let mut builder = PipelineBuilder::new();

        let layout1 = wgpu::VertexBufferLayout {
            array_stride: 12,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[],
        };

        let layout2 = wgpu::VertexBufferLayout {
            array_stride: 24,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[],
        };

        builder.add_buffer_layout(layout1);
        builder.add_buffer_layout(layout2);

        assert_eq!(builder.vertex_buffer_layouts.len(), 2);
        assert_eq!(builder.vertex_buffer_layouts[0].array_stride, 12);
        assert_eq!(builder.vertex_buffer_layouts[1].array_stride, 24);
        assert_eq!(
            builder.vertex_buffer_layouts[1].step_mode,
            wgpu::VertexStepMode::Instance
        );
    }

    #[test]
    fn test_builder_with_different_texture_formats() {
        let mut builder = PipelineBuilder::new();

        // Test various texture formats
        let formats = [
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::R32Float,
        ];

        for format in formats {
            builder.set_pixel_format(format);
            assert_eq!(builder.pixel_format, format);
        }
    }

    #[test]
    fn test_empty_shader_source() {
        let mut builder = PipelineBuilder::new();

        builder.set_shader_module("", "vs_main", "fs_main");

        assert_eq!(builder.shader_source, "");
    }

    #[test]
    fn test_builder_state_consistency() {
        let mut builder = PipelineBuilder::new();

        // Configure the builder
        builder.set_shader_module("// complex", "vertex_main", "fragment_main");
        builder.set_pixel_format(wgpu::TextureFormat::Rgba16Float);

        let layout = wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[],
        };
        builder.add_buffer_layout(layout);

        assert_eq!(builder.shader_source, "// complex");
        assert_eq!(builder.vertex_entry, "vertex_main");
        assert_eq!(builder.fragment_entry, "fragment_main");
        assert_eq!(builder.pixel_format, wgpu::TextureFormat::Rgba16Float);
        assert_eq!(builder.vertex_buffer_layouts.len(), 1);
        assert_eq!(builder.vertex_buffer_layouts[0].array_stride, 32);
    }
}
