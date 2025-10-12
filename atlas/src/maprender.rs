use crate::uniform::UniformBuffer;
use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use wgpu::{MultisampleState, RenderPipeline};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderGroup0Uniforms {
    pub time: f32,
    pub map_zoom: f32,
    pub map_translation: Vector2<u32>,
    pub window_size: Vector2<u32>,
}

unsafe impl Zeroable for RenderGroup0Uniforms {
    fn zeroed() -> Self {
        Self {
            time: 0.,
            map_zoom: 0.,
            map_translation: Vector2::new(0, 0),
            window_size: Vector2::new(0, 0),
        }
    }
}
unsafe impl Pod for RenderGroup0Uniforms {}

struct MapRenderer {
    pipeline: RenderPipeline,
    sampler: wgpu::Sampler,
    bind_group0: wgpu::BindGroup,
    bind_group1: wgpu::BindGroup,
}

impl MapRenderer {
    fn bind_group_samplers(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    visibility: wgpu::ShaderStages::FRAGMENT,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    visibility: wgpu::ShaderStages::FRAGMENT,
                },
            ],
        })
    }

    pub fn create_pipeline(&self, device: &wgpu::Device) -> RenderPipeline {
        let layout = wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &UniformBuffer::<RenderGroup0Uniforms>::bind_group_layout(device),
                &self.bind_group_samplers(device),
            ],
            ..Default::default()
        };
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            source: wgpu::ShaderSource::Wgsl(
                std::fs::read_to_string("../shaders/render_map.wgsl")
                    .unwrap()
                    .into(),
            ),
            label: None,
        });
        let rpl_desc = wgpu::RenderPipelineDescriptor {
            layout: Some(&device.create_pipeline_layout(&layout)),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertexMain"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragmentMain"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            cache: None,
            label: None,
            multiview: None,
        };
        device.create_render_pipeline(&rpl_desc)
    }
    pub fn reload_shaders(&mut self, device: &wgpu::Device) {
        self.pipeline = self.create_pipeline(device);
    }
}
