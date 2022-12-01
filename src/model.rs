use std::{fs, io::Read, mem, path::Path, borrow::Borrow};
use anyhow::Ok;
use wgpu::{util::DeviceExt, RenderPass, Buffer, BindGroupLayout};
use crate::{vmath, texture};

const RESOURCES_PATH: &str = "res";

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct Mesh {
    //pub vertices: Vec<Vertex>,
    pub indices:  Vec<u32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub material: Material,
}

pub struct Model {
    pub position: vmath::Vector3<f32>,
    pub meshes: Vec<Mesh>,
}

pub struct Material {
    pub texture: texture::Texture,
}

impl Model {
    pub fn new(file_name: &str, position: vmath::Vector3<f32>, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, anyhow::Error> {
        let path = Path::new(RESOURCES_PATH).join(file_name);
        let gltf = gltf::Gltf::open(path)?;

        let mut images = Vec::new();
        let mut materials = Vec::new();

        let mut buffers = Vec::new();
        let mut meshes: Vec<Mesh> = Vec::new();


        for image in gltf.images() {
            match image.source() {
                gltf::image::Source::Uri {
                    uri,
                    ..
                } => {
                    let mut file = fs::File::open(Path::new(RESOURCES_PATH).join(uri))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    images.push(buffer);
                }
                _ => {}
            }
        }

        for material in gltf.materials() {
            match material.pbr_metallic_roughness().base_color_texture() {
                Some(info) => {
                    let texture = info.texture();
                    
                    materials.push(
                        Material {
                            texture: texture::Texture::from_bytes(
                                &device,
                                &queue,
                                &images[texture.source().index()],
                                match texture.name() {
                                    Some(name) => name,
                                    None => ""
                                }
                            )?
                        }
                    )
                },
                None => {}
            }
        }

        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    let mut file = fs::File::open( Path::new(RESOURCES_PATH).join(uri))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)?;
                    buffers.push(buffer);
                }
                _ => {}
            }
        }


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

                let vertex_buffer = device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Model vertex buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }
                );
        
            
                let index_buffer = device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Model index buffer"),
                        contents: bytemuck::cast_slice(&indices),
                        usage: wgpu::BufferUsages::INDEX,
                    }
                );

                let material_index = primitive
                    .material().index().unwrap();

                

                meshes.push(Mesh { indices, vertex_buffer, index_buffer, material: materials[material_index] });
            }
        }

        /* let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            &images[4],
            &format!("{}_diffuse", file_name)
        )?;

        let material = Material {
            texture: diffuse_texture,
        }; */

        Ok(Model { position, meshes})
    }

    pub fn vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
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

pub trait DrawModel<'a> {
    fn draw_model(&mut self, model: &'a Model, device: &'a wgpu::Device, texture_bind_group_layout: &'a BindGroupLayout);
}

impl<'a> DrawModel<'a> for RenderPass<'a> {
    fn draw_model(&mut self, model: &'a Model, device: &'a wgpu::Device, texture_bind_group_layout: &'a BindGroupLayout) {
        for mesh in &model.meshes {
            //Привящываем набор ресурсов
            let diffuse_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&mesh.material.texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&mesh.material.texture.sampler),
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                }
            );

            self.set_bind_group(0, &diffuse_bind_group, &[]);

            self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            self.draw_indexed(0..mesh.indices.len() as _ , 0, 0..1);
        }
    }
}