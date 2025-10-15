use bytemuck::{NoUninit, Pod};

/// Updated External Data (UED):
/// a struct containing all data that is
/// updated on the outside. E.g.: time, camera, size constraints etc.
pub trait ColorRenderPass<UED: NoUninit + Pod> {
    fn create_pipeline(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline;
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        col_render_target: &wgpu::TextureView,
        dep_render_target: Option<&wgpu::TextureView>,
        ued: UED,
    );
}
