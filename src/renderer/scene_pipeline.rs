use crate::{
    core::{mesh_builder, FlatAtom, Mesh},
    renderer::{depth_texture::DepthTexture, pipeline::PipelineBuilder},
};

pub struct ScenePipeline {
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) sphere_mesh: Mesh,
}

impl ScenePipeline {
    pub fn new(
        device: &wgpu::Device,
        pixel_format: wgpu::TextureFormat,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let sphere_mesh = mesh_builder::make_sphere(device, 16, 16);

        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_buffer_layout(mesh_builder::SphereVertex::get_layout());
        pipeline_builder.add_buffer_layout(FlatAtom::get_instance_layout());
        pipeline_builder.set_shader_module(
            include_str!("../shaders/shader.wgsl"),
            "vs_main",
            "fs_main",
        );
        pipeline_builder.set_pixel_format(pixel_format);
        pipeline_builder.set_depth_format(DepthTexture::FORMAT);

        let render_pipeline = pipeline_builder.build_pipeline(device, &[camera_bind_group_layout]);

        Self {
            render_pipeline,
            sphere_mesh,
        }
    }
}
