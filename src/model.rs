use std::{fs, io::Read};

use anyhow::Ok;
use wgpu::util::DeviceExt;
use crate::{vmath, texture};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<u32>,
}

pub struct Model {
    pub position: vmath::Vector3<f32>,
    pub meshes: Vec<Mesh>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub material: Material,
}

pub struct Material {
    pub name: String,
    pub texture: texture::Texture,
}

impl Model {
    pub fn new(file_name: &str, position: vmath::Vector3<f32>, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, anyhow::Error> {

        let gltf = gltf::Gltf::open(file_name)?;
        let mut buffers = Vec::new();

        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    println!("uri: {}", uri);
                    let mut file = fs::File::open(format!("res/{uri}"))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    buffers.push(buffer);
                }
                _ => {}
            }
        }

        let mut meshes: Vec<Mesh> = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut tex_coords: Vec<[f32; 2]> = Vec::new();
                let mut indices: Vec<u32> = Vec::new();
                let mut vertices: Vec<Vertex> = Vec::new();

                if let Some(read_indices) = reader.read_indices() {
                    indices = read_indices.into_u32().collect::<Vec<_>>();
                }

                if let Some(iter) = reader.read_positions() {
                    positions = iter.collect::<Vec<_>>();
                }

                if let Some(read_tex_coords) = reader.read_tex_coords(0) {
                    tex_coords = read_tex_coords.into_f32().collect::<Vec<_>>();
                }

                for i in 0..positions.len() {
                    vertices.push(Vertex {
                        position: [
                            positions[i][0] * -1.0 + position.x, 
                            positions[i][1] + position.y, 
                            positions[i][2] + position.z,
                        ],
                        tex_coords: tex_coords[i],
                    });
                }

                meshes.push(Mesh { vertices, indices });
            }
        }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Model vertex buffer"),
                contents: bytemuck::cast_slice(&meshes[0].vertices),
                //contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

    
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Model index buffer"),
                contents: bytemuck::cast_slice(&meshes[0].indices),
                //contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let mut textures = Vec::new(); 

        for image in gltf.images() {
            match image.source() {
                gltf::image::Source::Uri {
                    uri,
                    mime_type
                } => {
                    let mut file = fs::File::open(format!("res/{uri}"))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    textures.push(buffer);
                    println!("uri: {}, mime_type: {}", uri, mime_type.unwrap());
                }
                _ => {}
            }
        }

        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            &textures[0],
            "electric.png"
        ).unwrap();

        let material = Material {
            name: "aaa".to_string(),
            texture: diffuse_texture,
        };

        Ok(Model { position, meshes, vertex_buffer, index_buffer, material })
    }

    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

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
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}
