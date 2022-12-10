use wgpu;

// Base bind groups: camera, ...
pub struct Common {
    pub layout: wgpu::BindGroupLayout, 
    //cam_view_proj: [[f32; 4]; 4],
}

impl Common {
    pub fn new(device: &wgpu::Device) -> Self {
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
                label: Some("common_bind_group_layout"),
            }
        );

        Self {
            layout: bind_group_layout
        }
    }

    pub fn bind_group(&self, device: &wgpu::Device, view_proj_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: view_proj_buffer.as_entire_binding(),
                }
            ],
            label: Some("common_bind_group"),
        })
    }
}