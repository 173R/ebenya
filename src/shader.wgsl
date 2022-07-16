// vertex shader

//Выходные данные вертексного шейдера
struct VertexOutput {
    // С помощью @builtin(position), указываем,
    // что это поле отвечает за коордианты вершин
    @builtin(position) clip_position: vec4<f32>,
};


// @vertex точка входа в вертексный шейдер
@vertex
fn vs_main(
    //Входное значение
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    //var - переменная будем изменяема но необходимо задать тип
    var out: VertexOutput;
    //let - тип выводится, значение не может изменяться
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
// @location(0) - сохранить выходное значение в первом color target 
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //Возвращаем цвет конкретного фрагмента
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}