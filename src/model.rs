use wgpu::VertexBufferLayout;

use crate::vmath::Vector3;

pub struct Mesh {
    vertexes: Vec<f32>,
    indices:  Vec<u32>,
}

pub struct Model {
    pub position: Vector3<f32>,
    pub meshes: Vec<Mesh>
}

impl Model {
    pub fn new(file_name: &str, position: Vector3<f32>) -> Self {
        let (models, materials) = tobj::load_obj(
            file_name,
            &tobj::LoadOptions::default()
        ).expect("Failed to OBJ load file");

        
        let mut meshes: Vec<Mesh> = Vec::new();

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            meshes.push(Mesh { vertexes: mesh.positions.clone(), indices: mesh.indices.clone() });
            
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

            /* for (i, m) in materials.iter().enumerate() {
                println!("material[{}].name = \'{}\'", i, m.name);
                println!(
                    "    material.Ka = ({}, {}, {})",
                    m.ambient[0], m.ambient[1], m.ambient[2]
                );
                println!(
                    "    material.Kd = ({}, {}, {})",
                    m.diffuse[0], m.diffuse[1], m.diffuse[2]
                );
                println!(
                    "    material.Ks = ({}, {}, {})",
                    m.specular[0], m.specular[1], m.specular[2]
                );
                println!("    material.Ns = {}", m.shininess);
                println!("    material.d = {}", m.dissolve);
                println!("    material.map_Ka = {}", m.ambient_texture);
                println!("    material.map_Kd = {}", m.diffuse_texture);
                println!("    material.map_Ks = {}", m.specular_texture);
                println!("    material.map_Ns = {}", m.shininess_texture);
                println!("    material.map_Bump = {}", m.normal_texture);
                println!("    material.map_d = {}", m.dissolve_texture);
        
                for (k, v) in &m.unknown_param {
                    println!("    material.{} = {}", k, v);
                }
            } */
        }


        Self { position, meshes }
    }

    pub fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}