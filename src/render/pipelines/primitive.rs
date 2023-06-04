use std::mem;
use wgpu;

use super::common;
use crate::texture;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl Vertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Base pipeline for primitive objects.
pub struct PrimitivePipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl PrimitivePipeline {
    pub fn new(
        device: &wgpu::Device,
        common: &common::Common,
        surface_config: &wgpu::SurfaceConfiguration
    ) -> Self {
        /*let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("primitive_shader"),
            source: wgpu::ShaderSource::Glsl {}
        });*/

        let primitive_shader = device.create_shader_module(
            wgpu::include_wgsl!("primitive.wgsl")
        );

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("primitive_render_pipeline_layout"),
                bind_group_layouts: &[
                    &common.layout
                ],
                push_constant_ranges: &[],
            }
        );

        Self {
            pipeline: device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: Some("primitive_render_pipeline"),
                    layout: Some(&render_pipeline_layout),
                    //@vertex
                    vertex: wgpu::VertexState {
                        module: &primitive_shader,
                        entry_point: "vs_main",
                        buffers: &[
                            Vertex::buffer_layout()
                            //model::Model::vertex_buffer_layout(),
                        ],
                    },
                    //@fragment
                    fragment: Some(wgpu::FragmentState {
                        module: &primitive_shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: surface_config.format,
                            //REPLACE - новые цвета замещают старые
                            blend: Some(wgpu::BlendState::REPLACE),
                            //Использовать все компоненты цвета, RGBA
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    //Как интерпретировать вершины при конвертации в треугольники
                    primitive: wgpu::PrimitiveState {
                        //Каждые три вершины будут соответствовать 
                        //одному треугольнику
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        //Треугольник обращён вперёд если 
                        //построен проти часовой стрелки
                        front_face: wgpu::FrontFace::Ccw,
                        //Те которые не обращены вперёд, не рендерятся
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        //Сколько сэмплов будет использовать конвейер
                        count: 1,
                        //Использовать все активные сэмплы
                        mask: !0,
                        // для сглаживания (пока отключено)
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                }
            )
        }

    }
}