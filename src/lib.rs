use wgpu::{util::DeviceExt};
use winit::{
    event::*,
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use crate::vmath::{Vector3, Matrix4x4};

mod vmath;
mod log;

mod texture;
mod camera;
mod instance;
mod model;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 1240.0;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    camera: camera::Camera,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,

    instances: Vec<instance::Instance>,
    instance_buffer: wgpu::Buffer,
    

    obj_model: model::Model
}

impl State {
    async fn new(window: &Window) -> Self {

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
        let diffuse_bytes = include_bytes!("electric.jpg");
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "electric.png"
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

        let mut camera = camera::Camera::new(
        vmath::Vector3::new(0.0, 0.0, 0.0),
            vmath::Vector3::new(0.0, 0.0, 1.0),
            60.0,
            WIDTH,
            HEIGHT
        );
        camera.update(instant::Duration::default());

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

        //Текстура глубины
        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");


        //INSTANCES//

        const NUM_INSTANCES_PER_ROW: u32 = 10;
        const INSTANCE_DISPLACEMENT: Vector3<f32> = 
            Vector3::new(
                NUM_INSTANCES_PER_ROW as f32 * 0.5,
                0.0,
                NUM_INSTANCES_PER_ROW as f32 * 0.5
            );

        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = Vector3::new(x as f32, 0.0, z as f32) - INSTANCE_DISPLACEMENT;

                let rotation = Matrix4x4::new_rotate(Vector3::unit_y(), 45.0);
            
            
                instance::Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data =
            instances.iter().map(instance::Instance::to_raw).collect::<Vec<_>>();
        
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        //INSTANCES//



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
                        //model::ModelVertex::desc(),
                        model::Model::buffer_layout(),
                        //instance::InstanceRaw::desc()
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
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


        let obj_model = model::Model::new(
            "res/cube.obj",
            Vector3::new(0.0, 0.0, 2.0),
            &device,
            model::ObjectFormat::GLTF
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            diffuse_bind_group,
            diffuse_texture,
            camera,
            //camera,
            //camera_uniform,
            //camera_buffer,
            camera_bind_group,
            camera_buffer,
            depth_texture,
            instances,
            instance_buffer,
            obj_model
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.width = new_size.width;
            self.surface.configure(&self.device, &self.config);
            //Обновление текстуры глубины
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { 
                input,
                .. 
            } => self.camera.keyboard_events(&input),
            _ => {}
        }
    
        
        return false;
    }

    fn update(&mut self, delta_time: instant::Duration) {
        self.camera.update(delta_time);
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
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                }
        );

            render_pass.set_pipeline(&self.render_pipeline);
            //группа с текстурами и семплером
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            //параметры: номер слота, вершины
            render_pass.set_vertex_buffer(0, self.obj_model.vertex_buffer.slice(..));
            //render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            //Можно использовать только оидн индексный буфер
            render_pass.set_index_buffer(self.obj_model.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            //нарисовать три вершины в одном экземляре
            //render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);

            render_pass.draw_indexed(0..self.obj_model.meshes[0].indices.len() as u32, 0, 0..1);
            
            //use model::DrawModel;
            //render_pass.draw_mesh_instanced(mesh, material, 0..self.instances.len() as u32, &self.camera_bind_group);
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
    let window = 
        WindowBuilder::new()
            .with_inner_size(PhysicalSize {
                height: HEIGHT,
                width: WIDTH,
            })
            .with_title("ebenya")
            .with_position(PhysicalPosition { x: 0, y: 0 })
            .build(&event_loop)
            .unwrap();
    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);
    window.set_cursor_position(PhysicalPosition::new(WIDTH * 0.5, HEIGHT * 0.5));

    #[cfg(target_arch = "wasm32")]
    {
        window.set_inner_size(PhysicalSize::new(WIDTH, HEIGHT));
        //window.set_cursor_visible(false);
        
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
    let mut last_render_time = instant::Instant::now();

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
                    new_inner_size, 
                    ..
                } => state.resize(**new_inner_size),
                _ => {}
            }
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let now = instant::Instant::now();
            let delta_time = now - last_render_time;
            last_render_time = now;

            state.update(delta_time);
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
        Event::DeviceEvent {  
            event: DeviceEvent::MouseMotion {
                delta
            },
            ..
        } => state.camera.mouse_events(delta.0 as f32, delta.1 as f32),
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        log::write("test", "text", None).unwrap();
        //let obj = model::Model::new("res/keytruck.obj", Vector3::new(0.0, 0.0, 0.0));
    }
}