use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use wgpu::RenderPipeline;

use crate::{renderpass::ColorRenderPass, uniform::UniformBuffer};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct FigureUniform {
    figure_origin: [Vector2<f32>; 16],
    zoom: [f32; 16],
    window_size: Vector2<f32>,
    figure_extent: Vector2<f32>,

    time: f32,
    mouse_pos: f32,
    dragged: i32,
}
struct SamplerData {
    bind_group: wgpu::BindGroup,
}
impl SamplerData {
    //TODO
}
pub struct FigureRenderpass {
    pub pipeline: RenderPipeline,
    uniform: Option<UniformBuffer<FigureUniform>>,
    color_format: wgpu::TextureFormat,
}
impl FigureRenderpass {
    fn new(
        figures: Vec<&str>,
        color_format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // TODO:
        // 1. build a texture atlas from the figures
        // 2. load the texture atlas into memory
        // 3. set up the render pass
        todo!()
    }
    fn reload_shaders(&mut self, device: &wgpu::Device) {
        self.pipeline = Self::create_pipeline(device, self.color_format);
    }
}

impl ColorRenderPass<FigureUniform> for FigureRenderpass {
    fn create_pipeline(device: &wgpu::Device, cf: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        todo!()
    }
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        surfacetv: &wgpu::TextureView,
        _: Option<&wgpu::TextureView>,
        ued: FigureUniform,
    ) {
        todo!()
    }
}
