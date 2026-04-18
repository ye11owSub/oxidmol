use std::{env::current_dir, fs};

pub struct PipelineBuilder {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
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
            shader_filename: "dummy".to_string(),
            vertex_entry: "dummy".to_string(),
            fragment_entry: "dummy".to_string(),
            pixel_format: wgpu::TextureFormat::Rgba8Unorm,
            vertex_buffer_layouts: Vec::new(),
        }
    }

    pub fn set_shader_module(
        &mut self,
        shader_filename: &str,
        vertex_entry: &str,
        fragment_entry: &str,
    ) {
        self.shader_filename = shader_filename.to_string();
        self.vertex_entry = vertex_entry.to_string();
        self.fragment_entry = fragment_entry.to_string();
    }

    pub fn set_pixel_format(&mut self, pixel_format: wgpu::TextureFormat) {
        self.pixel_format = pixel_format;
    }

    pub fn add_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout)
    }

    pub fn build_pipeline(&mut self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let mut file_path = current_dir().unwrap();
        file_path.push("src/");
        file_path.push(self.shader_filename.as_str());
        let file_path = file_path.into_os_string().into_string().unwrap();
        let source_code = fs::read_to_string(file_path).expect("Can't read source code");

        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

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
                buffers: &self.vertex_buffer_layouts,
            },

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some(&self.fragment_entry),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &render_targets,
            }),

            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
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

        assert_eq!(builder.shader_filename, "dummy");
        assert_eq!(builder.vertex_entry, "dummy");
        assert_eq!(builder.fragment_entry, "dummy");
        assert_eq!(builder.pixel_format, wgpu::TextureFormat::Rgba8Unorm);
        assert!(builder.vertex_buffer_layouts.is_empty());
    }

    #[test]
    fn test_set_shader_module() {
        let mut builder = PipelineBuilder::new();

        builder.set_shader_module("test_shader.wgsl", "vs_main", "fs_main");

        assert_eq!(builder.shader_filename, "test_shader.wgsl");
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
    fn test_empty_shader_filename() {
        let mut builder = PipelineBuilder::new();

        builder.set_shader_module("", "vs_main", "fs_main");

        assert_eq!(builder.shader_filename, "");
    }

    #[test]
    fn test_builder_state_consistency() {
        let mut builder = PipelineBuilder::new();

        // Настраиваем builder
        builder.set_shader_module("complex_shader.wgsl", "vertex_main", "fragment_main");
        builder.set_pixel_format(wgpu::TextureFormat::Rgba16Float);

        let layout = wgpu::VertexBufferLayout {
            array_stride: 32,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[],
        };
        builder.add_buffer_layout(layout);

        assert_eq!(builder.shader_filename, "complex_shader.wgsl");
        assert_eq!(builder.vertex_entry, "vertex_main");
        assert_eq!(builder.fragment_entry, "fragment_main");
        assert_eq!(builder.pixel_format, wgpu::TextureFormat::Rgba16Float);
        assert_eq!(builder.vertex_buffer_layouts.len(), 1);
        assert_eq!(builder.vertex_buffer_layouts[0].array_stride, 32);
    }
}
