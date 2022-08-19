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
pub struct CameraMovement {
    left: bool,
    right: bool,
    forward: bool,
    backward: bool,
}

#[derive(Debug)]
pub struct Camera {
    pub uniform: CameraUniform,
    position: Vector3<f32>,
    target: Vector3<f32>,
    fov: f32,
    width: f32, 
    height: f32,
    movement: CameraMovement,
    speed: f32,
    //buffer: Option<wgpu::Buffer>,
    //bind_group: Option<wgpu::BindGroup>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, target: Vector3<f32>, fov: f32, width: f32, height: f32) -> Self {
        Self {
            uniform: CameraUniform {
                view_proj: Matrix4x4::new_indent().into(),
            },
            position,
            target,
            fov,
            width,
            height,
            movement: CameraMovement {
                left: false,
                right: false,
                forward: false,
                backward: false,
            },
            speed: 0.1,

        }
    }

    pub fn update(&mut self) {

        let mut dir: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        if self.movement.left {
            dir.x = -1.0;
        }

        if self.movement.right {
            dir.x = 1.0;
        }

        if self.movement.forward {
            dir.z = 1.0;
        }

        if self.movement.backward {
            dir.z = -1.0;
        }

        self.position = self.position + dir * self.speed;

        let view = lookAt(self.position, self.target);
        let proj = Matrix4x4::new_perspective(
            self.width, self.height, 0.1, 100.0, self.fov
        );
        self.uniform.view_proj = (proj * view).into();
        //println!("cam.pos = {:?}", self.position);
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

    pub fn poll_events(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode,
                        ..
                    },
                    ..
            } => {
                let pressed = *state == ElementState::Pressed;
                match virtual_keycode {
                    Some(VirtualKeyCode::A | VirtualKeyCode::Left) => {
                        self.movement.left = pressed;      
                    },
                    Some(VirtualKeyCode::D | VirtualKeyCode::Right) => {
                        self.movement.right = pressed;  
                    },
                    Some(VirtualKeyCode::W | VirtualKeyCode::Up) => {
                        self.movement.forward = pressed;    
                    },
                    Some(VirtualKeyCode::S | VirtualKeyCode::Down) => {
                        self.movement.backward = pressed;    
                    },
                    _ => {}
                }
            },
            _ => {}
            // WindowEvent::CursorMoved { 
            //     position,
            //     ..
            // } => { /*println!("cursor: {:?}", position)*/},
            // _ => {}
        }
    }
}