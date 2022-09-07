use wgpu::util::DeviceExt;
use crate::vmath::Vector3;

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
    pub position: Vector3<f32>,
    pub meshes: Vec<Mesh>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

pub enum ObjectFormat {
    OBJ,
    GLTF,
}

impl Model {
    pub fn new(file_name: &str, position: Vector3<f32>, device: &wgpu::Device, format: ObjectFormat) -> Self {
    
        let mut meshes: Vec<Mesh> = Vec::new();

        match format {
            ObjectFormat::GLTF => {
                let (gltf, buffers, _) = gltf::import("res/electric.gltf").unwrap();
                for mesh in gltf.meshes() {
                    for primitive in mesh.primitives() {
                        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                        //let texture = primitive.material().te;

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
                                position: [positions[i][0] * -1.0, positions[i][1], positions[i][2]],
                                tex_coords: tex_coords[i],
                            });
                        }

                        meshes.push(Mesh { vertices, indices });
                    }
                 }
            },
            ObjectFormat::OBJ => {

            },
            _ => {}
        }

        /* let (models, materials) = tobj::load_obj(
            file_name,
            &tobj::LoadOptions::default()
        ).expect("Failed to OBJ load file");

        
    

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            meshes.push(Mesh {
                vertices: (0..mesh.positions.len() / 3).map(|i| Vertex {
                    position: [mesh.positions[i * 3] + position.x, mesh.positions[i * 3 + 1] + position.y, mesh.positions[i * 3 + 2] + position.z],
                    tex_coords: [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]],
                }).collect::<Vec<_>>(),
                indices: mesh.indices.clone()
            });
            
            let mut next_face = 0;
            for face in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[face] as usize;

                let face_indices = &mesh.indices[next_face..end];

                if !mesh.texcoord_indices.is_empty() {
                    let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                }
                if !mesh.normal_indices.is_empty() {
                    let normal_face_indices = &mesh.normal_indices[next_face..end];
                }
    
                next_face = end;
            }

            assert!(mesh.positions.len() % 3 == 0);
        } */

        //let test = &meshes[0].vertexes[0..=meshes[0].vertexes.len() / 3];

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

        Self { 
            position, 
            meshes, 
            vertex_buffer,
            index_buffer
        }
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