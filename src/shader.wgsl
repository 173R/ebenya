// vertex shader

// struct InstanceInput {
//     @location(5) model_matrix_0: vec4<f32>,
//     @location(6) model_matrix_1: vec4<f32>,
//     @location(7) model_matrix_2: vec4<f32>,
//     @location(8) model_matrix_3: vec4<f32>,
// }


struct CameraUniform {
   view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) tex_coords: vec2<f32>,
// }

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

//Выходные данные вертексного шейдера
struct VertexOutput {
    // С помощью @builtin(position), указываем,
    // что это поле отвечает за коордианты вершин
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};


// @vertex точка входа в вертексный шейдер
@vertex
fn vs_main(
    model: VertexInput,
    //instance: InstanceInput
) -> VertexOutput {
    // let model_matrix = mat4x4<f32>(
    //     instance.model_matrix_0,
    //     instance.model_matrix_1,
    //     instance.model_matrix_2,
    //     instance.model_matrix_3,
    // );
    //var - переменная будем изменяема но необходимо задать тип
    var out: VertexOutput;
    //let - тип выводится, значение не может изменяться
    //out.clip_position =  camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    
    out.clip_position =  camera.view_proj * vec4<f32>(model.position, 1.0);
    

    //out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

// Fragment shader
//uniform переменные
// group - первый параметр в set_bind_group()
// @binding - параметр в create_bind_group()
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
// @location(0) - сохранить выходное значение в первом color target 
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //Возвращаем цвет конкретного фрагмента
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    //return vec4(0.8, 0.0, 0.1, 1.0);
}

//ПОлучается в wgpu перемножение major col
//Получается wgsl принимает массив который интерпретирует в mat4x4<f32>, вектор столбцов.
//То есть на вход даём [[1.0, 0.0, 0.0, 2.0][0.0, 1.0, 0.0, 0.0][0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
//У меня юзается row-major система поэтому в у меня он имеет представление:
//[1.0, 0.0, 0.0, 2.0]
//[0.0, 1.0, 0.0, 0.0]
//[0.0, 0.0, 1.0, 0.0]
//[0.0, 0.0, 0.0, 1.0]

//А wgsl преобразует этот же массив как:
//[1.0, 0.0, 0.0, 0.0]
//[0.0, 1.0, 0.0, 0.0]
//[0.0, 0.0, 1.0, 0.0]
//[2.0, 0.0, 0.0, 1.0]
//То есть как массив столбцов а не строк
