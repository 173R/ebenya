use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout};
use crate::vmath::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view: [[f32; 4]; 4],
}

pub struct Camera {
    //position: Vector3<f32>,
    //target: Vector3<f32>,
    //up: Vector3<f32>,
    //fov: f32,
    //eye: f32,
    //target: f32,
    uniform: CameraUniform,

    //buffer: Option<wgpu::Buffer>,
    //bind_group: Option<wgpu::BindGroup>,
}

impl Camera {
    pub fn new(a: &Matrix4x4) -> Self {
        Self {
            uniform: CameraUniform {
                view: a.into()
            }, 
            //buffer: None,
            //bind_group: None
        }
    }

    fn update_view(&mut self) {
        //self.camera_uniform.view = view.into();
    }

    pub fn get_camera_bind_groups(&mut self, device: &wgpu::Device) -> (
        BindGroupLayout,
        BindGroup
    ) {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[self.uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        (bind_group_layout, bind_group)
    }
}