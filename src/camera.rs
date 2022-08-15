use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer};
use winit::{
    event::*,
};
use crate::vmath::{Vector3, Matrix4x4, lookAt};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[derive(Debug)]
pub struct Camera {
    pub uniform: CameraUniform,
    position: Vector3<f32>,
    target: Vector3<f32>,
    fov: f32,
    width: f32, 
    height: f32,
    


    //up: Vector3<f32>,
    //fov: f32,
    //eye: f32,

    //buffer: Option<wgpu::Buffer>,
    //bind_group: Option<wgpu::BindGroup>,
    //perspective: Matrix4x4<f32>,  
}

impl Camera {
    pub fn new(position: Vector3<f32>, target: Vector3<f32>, fov: f32, width: f32, height: f32) -> Self {
        Self {
            uniform: CameraUniform {
                view_proj: Matrix4x4::new_indent().into(),
            },

            //perspective: Matrix4x4::new_perspective(800.0, 600.0, 0.1, 100.0, 90.0),
            //buffer: None,
            //bind_group: None
            position,
            target,
            fov,
            width,
            height,

        }
    }

    pub fn update_view_proj(&mut self) {
        let view = lookAt(self.position, self.target);
        let proj = Matrix4x4::new_perspective(
            self.width, self.height, 0.1, 100.0, self.fov
        );
        self.uniform.view_proj = (proj * view).into();
        println!("cam.pos = {:?}", self.position);
    }

    pub fn get_camera_bind_groups(&mut self, device: &wgpu::Device) -> (
        BindGroupLayout,
        BindGroup,
        Buffer,
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

        (bind_group_layout, bind_group, buffer)
    }

    pub fn move_left(&mut self) {
        self.position = Vector3::new(self.position.x - 0.1, self.position.y, self.position.z);
        self.update_view_proj();
    }

    pub fn move_right(&mut self) {
        self.position = Vector3::new(self.position.x + 0.1, self.position.y, self.position.z);
        self.update_view_proj();
    }

    pub fn move_forward(&mut self) {
        self.position = Vector3::new(self.position.x, self.position.y, self.position.z + 0.1);
        self.update_view_proj();
    }

    pub fn move_backward(&mut self) {
        self.position = Vector3::new(self.position.x, self.position.y, self.position.z - 0.1);
        self.update_view_proj();
    }

    pub fn poll_events(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::A),
                        ..
                    },
                    ..
            } => {
                self.move_left();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::D),
                        ..
                    },
                    ..
            } => {
                self.move_right();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::W),
                        ..
                    },
                    ..
            } => {
                self.move_forward();
            },
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::S),
                        ..
                    },
                    ..
            } => {
                self.move_backward();
            },
            WindowEvent::CursorMoved { 
                position,
                ..
            } => { println!("cursor: {:?}", position)},
            _ => {}
        }
    }
}