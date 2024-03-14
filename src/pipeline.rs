use wgpu::util::DeviceExt;

use crate::device::Device;

pub struct Pipeline {
    layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &Device, 
        layout_descriptor: wgpu::PipelineLayoutDescriptor, 
        shader_path: wgpu::ShaderModuleDescriptor,
        //bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {
        // let layout = device.create_pipeline_layout(
        //     &wgpu::PipelineLayoutDescriptor {
        //         label: Some("Render Pipeline Layout"),
        //         bind_group_layouts: bind_group_layouts,
        //         push_constant_ranges: &[],
        //     }
        // );
        let pipeline = create_render_pipeline(
            &device.device,
            &layout,
            config.format,
            Some(texture::Texture::DEPTH_FORMAT),
            &[model::ModelVertex::desc(), InstanceRaw::desc()],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
        );
        Self {
            layout: layout,
            pipeline: pipeline,
        }
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    topology: wgpu::PrimitiveTopology,
    shader:  wgpu::ShaderModuleDescriptor,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::REPLACE,
                    color: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}