use std::f32::consts::FRAC_PI_2; use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer}; use winit::{ event::*, };
use crate::vmath::{Vector3, Matrix4x4};

#[derive(Debug, PartialEq)]
enum CameraMode {
    Player,
    Free,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[derive(Debug)]
pub struct CameraMovement {
    forward: f32,
    backward: f32,
    left: f32,
    right: f32,
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
    rotate_x: f32,
    rotate_y: f32,
    sensitivity: f32,
    yaw: f32,
    pitch: f32,
    mode: CameraMode,
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
                forward: 0.0,
                backward: 0.0,
                right: 0.0, left: 0.0, }, speed: 10.0, rotate_x: 0.0, rotate_y: 0.0, yaw: 0.0, pitch: 0.0, sensitivity: 0.4,
            mode: CameraMode::Free

        }
    }

    pub fn update(&mut self, delta_time: instant::Duration) {

        //println!("yaw: {:?}, pitch: {:?}", self.yaw.to_degrees(), self.pitch.to_degrees());

        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        self.target = Vector3::new(yaw_sin * pitch_cos, -pitch_sin, pitch_cos * yaw_cos).normalize(); 

        let right = Vector3::unit_y().cross(self.target) * (self.movement.right - self.movement.left);
        let mut forward = self.target * (self.movement.forward - self.movement.backward);
        if self.mode == CameraMode::Player {
            forward.y = 0.0;
        }

        self.position = self.position + (right + forward) * self.speed * delta_time.as_secs_f32();

        let view = Matrix4x4::new_look_at(self.position, self.target);
        let proj = Matrix4x4::new_perspective(
            self.width, self.height, 0.1, 1000.0, self.fov
        );

        self.uniform.view_proj = (proj * view).into();

        self.yaw += self.rotate_x * self.sensitivity * delta_time.as_secs_f32();
        self.pitch += self.rotate_y * self.sensitivity * delta_time.as_secs_f32();

        if self.pitch > FRAC_PI_2 {
            self.pitch = FRAC_PI_2;
        }

        if self.pitch < -FRAC_PI_2 {
            self.pitch = -FRAC_PI_2;
        }

        self.rotate_x = 0.0;
        self.rotate_y = 0.0;

        
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

    pub fn keyboard_events(&mut self, input: &KeyboardInput) {
        let offset: f32 = if input.state == ElementState::Pressed { 1.0 } else { 0.0 };

        match input.virtual_keycode {
            Some(VirtualKeyCode::A | VirtualKeyCode::Left) => {
                self.movement.left = offset;      
            },
            Some(VirtualKeyCode::D | VirtualKeyCode::Right) => {
                self.movement.right = offset;  
            },
            Some(VirtualKeyCode::W | VirtualKeyCode::Up) => {
                self.movement.forward = offset;    
            },
            Some(VirtualKeyCode::S | VirtualKeyCode::Down) => {
                self.movement.backward = offset;    
            },
            _ => {}
        }
    }

    pub fn mouse_events(&mut self, delta_x: f32, delta_y: f32,) {
        self.rotate_x = delta_x;
        self.rotate_y = delta_y;
    }
}
