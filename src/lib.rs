use wgpu::util::DeviceExt;
use vmath::{
    Matrix4x4
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


mod texture;
mod vmath;
mod camera;


#[cfg(test)]
mod tests {
    use crate::vmath::Matrix4x4;

    use super::*;
    #[test]
    fn it_works() {
        let matrix: Matrix4x4<f32> =
            vmath::Matrix4x4::new_indent();

        // let matrix_s: Matrix4x4<f32> =
        //     vmath::Matrix4x4::new_translation(&[2.1, 2.2, 2.3, 1.0]);

        let matrix_t: Matrix4x4<f32> =
            vmath::Matrix4x4::new_scale(&[2.1, 2.2, 2.3, 1.0]);

        let aaa = 
            vmath::Vector3::new(1.0, 0.0, 0.0).cross(vmath::Vector3::new(0.0, 0.0, 1.0)); 
        let vect = vmath::Vector3::new(-5.0, 10.0, -2.0).normalize();
        let vect2 = vmath::Vector3::new(5.0, 10.0, 2.0).normalize();


        //let matrix_t: Matrix4x4<f32> =
        //    vmath::Matrix4x4::new_perspective(2560.0, 1440.0, );

        //println!("{:?}", matrix);
        // println!("{:?}", matrix_s);
        // println!("{:?}", matrix_t);
        // println!("{:?}", matrix * matrix_s);


        //cgmath::Vector3
        //cgmath::Matrix4::identity().into();
        
        //println!("{:?}", matrix * matrix_s);

        //println!("{:?}", matrix_s);
       //nalgebra::Matrix4::new_orthographic(left, right, bottom, top, znear, zfar)
       //dbg!(10);
    }
}

/*
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

*/

/*#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
*/

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        //Определяем как буфер будет распределён в памяти
        wgpu::VertexBufferLayout {
            //шаг или ширина вертекса (24 байта)
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            //Описание каждой части вершины
            attributes: &[
                wgpu::VertexAttribute {
                    //начальное смещение
                    offset: 0,
                    //Задаём соответсвие локации к набору данных
                    shader_location: 0,
                    //Форма данных
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [2.0, -2.0, 10.0], tex_coords: [1.0, 1.0], }, // A
    Vertex { position: [-2.0, -2.0, 4.0], tex_coords: [0.0, 1.0], }, // A
    Vertex { position: [-2.0, 2.0, 4.0], tex_coords: [0.0, 0.0], }, // A
    Vertex { position: [2.0, 2.0, 10.0], tex_coords: [1.0, 0.0], }, // A
    
    //Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    //Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    //Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    //Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    //Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
    0, 3, 2,
    2, 1, 0,
];

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    space_on: bool,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    second_diffuse_bind_group: wgpu::BindGroup,
    camera: camera::Camera,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl State {
    async fn new(window: &Window) -> Self {

        //let matrix: vmath::Matrix4x4<f64> = vmath::Matrix4x4::new();

        /////
        let size = window.inner_size();

        //Инстанс самого wgpu
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        //Поверхность для отображения
        let surface = unsafe { instance.create_surface(window)};
        //Хэндлер видекарты
        let adapter = instance.request_adapter( 
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false, // true - максимальная поддержка всех платформ, софтварный рендер
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") { // задаём лимиты для максимальной поддержки всех бэкендов
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();


        //Конфигурируем surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, //Текстуры будут юзаться для вывода на экран
            format: surface.get_supported_formats(&adapter)[0], // как будут хранится SurfaceTexture в GPU
            width: size.width,
            height: size.height,
            //Формат синхронизации поверхности с дисплеем, Fifo ограничивает частоту кадров, поодерживается везде, по сути это VSync
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        //Загружаем изображение
        let diffuse_bytes = include_bytes!("cat.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "cat.png"
        ).unwrap();
        

        let second_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            include_bytes!("blue_texture.png"),
            "blue_texture.png"
        ).unwrap();

        //Описываем набор ресурсов и то,
        //как к ним пожно получить доступ из шейдера
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );

        //Привязываем набор ресурсов
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let second_diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&second_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&second_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        //let view_matrix: Matrix4x4<f32> = Matrix4x4::new_indent();

        let mut camera = camera::Camera::new(
        vmath::Vector3::new(0.0, 0.0, 0.0),
            vmath::Vector3::new(0.0, 0.0, 1.0),
            90.0,
            800.0,
            600.0
        );
        camera.update();

        let (camera_bind_group_layout, camera_bind_group, camera_buffer) = 
            camera.get_camera_bind_groups(&device);


        let clear_color = wgpu::Color::WHITE;

        //Создаём шейдерный модуль
        /*let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    //Считываем файл в виде строки
                    include_str!("shader.wgsl").into(),
                )
            }
        );*/

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );

        //Создаём графичсекий конвейер

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                //Описание bind группы с текстурами,
                //которые уже загружаются в методе render
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,    
                ],
                push_constant_ranges: &[],
            }
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                //@vertex
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    //Указываем структуру буфера
                    buffers: &[
                        Vertex::desc(),
                    ],
                },
                //@fragment
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        //REPLACE - новые цвета замещают старые
                        blend: Some(wgpu::BlendState::REPLACE),
                        //Использовать все компоненты цвета, RGBA
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                //Как интерпретировать вершины при конвертации в треугольники
                primitive: wgpu::PrimitiveState {
                    //Каждые три вершины будут соответствовать 
                    //одному треугольнику
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    //Треугольник обращён вперёд если 
                    //построен проти часовой стрелки
                    front_face: wgpu::FrontFace::Ccw,
                    //Те которые не обращены вперёд, не рендерятся
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    //Сколько сэмплов будет использовать конвейер
                    count: 1,
                    //Использовать все активные сэмплы
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        //Создаём буфер вершин
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                //Преобразуем вершины в формат &[u8]
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;

        let space_on = false;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            space_on,
            diffuse_bind_group,
            diffuse_texture,
            second_diffuse_bind_group,
            camera,
            //camera,
            //camera_uniform,
            //camera_buffer,
            camera_bind_group,
            camera_buffer
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.width = new_size.width;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        // match event {
        //     WindowEvent::CursorMoved {
        //         position,
        //         ..
        //     } => {
        //         self.clear_color.r = position.x / self.size.width as f64;
        //         self.clear_color.g = position.y / self.size.height as f64;
        //         self.clear_color.b = (position.x + position.y) / (self.size.width + self.size.height) as f64;
        //         self.clear_color.a = 1.0;    
        //     },
        //     WindowEvent::KeyboardInput {
        //         input:
        //                 KeyboardInput {
        //                     state: ElementState::Released,
        //                     virtual_keycode: Some(VirtualKeyCode::Space),
        //                     ..
        //                 },
        //             ..
        //     } => {
        //         self.space_on = !self.space_on;
        //     },
        //     _ => {}
        // }

        self.camera.poll_events(event);
        
        return false;
    }

    fn update(&mut self) {
        self.camera.update();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform])
        )
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //Достаём текстуру из поверхности
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );
        //Кодировщик нужен для создания буфера команд которые потом пойдут в GPU
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        //Создаём проход рендера
        
        {//так как созданный _render_pass владеет encoder 
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        //Сохраняем цвета в созданную текстуру view
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            //Указываем как обрабатывать цвета которые остались в пердыдущем кадре
                            //В нашем случае мы их очищаем, закрашивая всё цветом
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            //Сохранять результат в текстуре view
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                }
        );

            render_pass.set_pipeline(&self.render_pipeline);
            //группа с текстурами и семплером
            if self.space_on {
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            } else {
                render_pass.set_bind_group(0, &self.second_diffuse_bind_group, &[]);    
            }
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            //параметры: номер слота, вершины
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            //Можно использовать только оидн индексный буфер
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            //нарисовать три вершины в одном экземляре
            if !self.space_on {
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            } else {
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }
        //Завершить буфер команд и отправить его в очередь
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(800, 600));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("ebenya")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged {
                    new_inner_size, ..
                } => state.resize(**new_inner_size),
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::MainEventsCleared => {
            //Получается этот ивент тригерится первый раз при создании окна??
            //И потом запрашивает перерисовку
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            //println!("aaaa");
            //Явно запрашиваем перерисовку
            window.request_redraw();
        }
        _ => {}
    });
}
