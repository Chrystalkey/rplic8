use crate::{renderpass::ColorRenderPass, uniform::UniformBuffer};
use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use wgpu::{MultisampleState, RenderPipeline};

const SHADER_NAME: &str = "shaders/render_map.wgsl";
const IMAGE_NAME: &str = "images/Unterbaucheingeweide 01 (73 x 65).png";

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Metadata {
    pub time: f32,
    pub map_zoom: f32,
    pub map_translation: Vector2<f32>,
    pub window_size: Vector2<f32>,
    pub mouse_pos: Vector2<f32>,
}

struct SamplerData {
    sampler: wgpu::Sampler,
    texture: wgpu::Texture,
    texview: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

impl SamplerData {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        // For more texture loading see: https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#getting-data-into-a-texture
        let image: image::RgbaImage = image::load(
            std::io::BufReader::new(
                std::fs::File::open(IMAGE_NAME).unwrap(),
            ),
            image::ImageFormat::Png,
        )
        .unwrap().into();
        let (w, h) = image.dimensions();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            size: wgpu::Extent3d {
                depth_or_array_layers: 1,
                width: w,
                height: h,
            },
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d::ZERO,
            },
            image.into_raw().as_slice(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                depth_or_array_layers: 1,
                width: w,
                height: h,
            },
        );

        let texview = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToBorder,
            address_mode_v: wgpu::AddressMode::ClampToBorder,
            address_mode_w: wgpu::AddressMode::ClampToBorder,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &Self::bglayout(device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texview),
                },
            ],
        });
        Self {
            sampler,
            texture,
            texview,
            bind_group,
        }
    }

    fn bglayout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
}

pub struct MapRenderpass {
    pub pipeline: RenderPipeline,
    uniforms: Option<UniformBuffer<Metadata>>,
    sampler_data: SamplerData,
    color_format: wgpu::TextureFormat,
}

impl MapRenderpass {
    pub fn new(
        color_format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let sampler_data = SamplerData::new(device, queue);
        Self {
            pipeline: Self::create_pipeline(device, color_format),
            uniforms: None,
            sampler_data,
            color_format,
        }
    }

    pub fn get_new_bind_groups(
        &mut self,
        uniforms: Metadata,
        device: &wgpu::Device,
    ) -> [&wgpu::BindGroup; 2] {
        self.uniforms = Some(UniformBuffer::new(device, uniforms, None));
        [
            self.uniforms.as_ref().unwrap().bind_group(),
            &self.sampler_data.bind_group,
        ]
    }
    pub fn reload_shaders(&mut self, device: &wgpu::Device){
        self.pipeline = Self::create_pipeline(device, self.color_format);
    }
}

impl ColorRenderPass<Metadata> for MapRenderpass {
    fn create_pipeline(device: &wgpu::Device, color_format: wgpu::TextureFormat) -> RenderPipeline {
        let layout = wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &UniformBuffer::<Metadata>::bind_group_layout(device),
                &SamplerData::bglayout(device),
            ],
            ..Default::default()
        };
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            source: wgpu::ShaderSource::Wgsl(
                std::fs::read_to_string(SHADER_NAME)
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
                    format: color_format,
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

    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        surfacetv: &wgpu::TextureView,
        _: Option<&wgpu::TextureView>,
        ued: Metadata,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Map Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                depth_slice: None,
                ops: wgpu::Operations {
                    store: wgpu::StoreOp::Store,
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                },
                resolve_target: None,
                view: surfacetv,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        for (idx, &bg) in self.get_new_bind_groups(ued, device).iter().enumerate() {
            pass.set_bind_group(idx as u32, bg, &[]);
        }
        pass.set_pipeline(&self.pipeline);
        pass.draw(0..4, 0..1);
    }
}
