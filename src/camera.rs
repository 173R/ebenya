use wgpu::util::DeviceExt;
use crate::vmath::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view: [[f32; 4]; 4],
}

pub struct Camera {
    position: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
    fov: f32,
    //eye: f32,
    //target: f32,
    uniform: CameraUniform,  

    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Camera {
    fn update_view(&mut self) {
        //self.camera_uniform.view = view.into();
    }

    fn get_camera_bind_group(&mut self, device: &wgpu::Device) {
        self.buffer = device.create_buffer_init(
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

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
    }
}