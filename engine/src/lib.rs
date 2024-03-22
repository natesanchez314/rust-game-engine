mod game;
mod camera;
mod texture;
mod model;
mod resources;
mod scene;
mod ecs;
mod my_window;
mod device;
mod command;
mod pipeline;

use camera::{camera3d::{Camera3d, Camera3dController}, CameraUniform, Projection};
use ecs::{component::component_3d::Transform3d, entity_manager::EntityManager};
use my_window::MyWindow;
//use pipeline::Pipeline;
use texture::Texture;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize, event::*, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}
};
use cgmath::prelude::*;
use model::*;

use crate::{device::Device, ecs::component::{Component, InstanceRaw}};

const NUM_INSTANCES_PER_ROW: u32 = 10;
const MAX_ENTITIES: u32 = 1000;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 3],
    _padding: u32,
    color: [f32; 3],
    _padding2: u32,
}

struct State {
    window: MyWindow,
    device: Device,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera3d,
    projection: Projection,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: Camera3dController,
    transform3ds: Vec<Transform3d>,
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
    obj_model: model::Model,
    light_uniform: LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group_layout: wgpu::BindGroupLayout,
    light_bind_group: wgpu::BindGroup,
    light_render_pipeline: wgpu::RenderPipeline,
    mouse_pressed: bool,

    //ECS 
    //entity_manager: EntityManager,
    //entities: Vec<Entity>,
    //transform_components: Vec<Transform2d>
    //model_components: Vec<ModelComponent>,
}

impl State {
    async fn new(window: MyWindow) -> Self {
        let size = PhysicalSize::new(window.width, window.height);
        let device = Device::new(&window).await;

        let surface_caps = device.surface.get_capabilities(&device.adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.width,
            height: window.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        device.surface.configure(&device.device, &config);

        let texture_bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture { 
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                        view_dimension: wgpu::TextureViewDimension::D2, 
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let camera = Camera3d::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection = camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_controller = camera::camera3d::Camera3dController::new(4.0, 0.4);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = device.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: None,
        });

        let camera_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group")
        });

        let depth_texture = texture::Texture::create_depth_texture(&device.device, &config, "depth_texure");

        let light_uniform = LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };

        let light_buffer = device.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let light_bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { 
                    ty: wgpu::BufferBindingType::Uniform, 
                    has_dynamic_offset: false, 
                    min_binding_size: None 
                },
                count: None,
            }],
            label: None,
        });

        let light_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let render_pipeline_layout = device.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            }
        );

        let render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
            };
            pipeline::create_render_pipeline(
                &device.device,
                &render_pipeline_layout,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc(), InstanceRaw::desc()],
                wgpu::PrimitiveTopology::TriangleList,
                shader,
            )
        };

        // let render_pipeline = Pipeline::new(&
        //     device, 
        //     wgpu::PipelineLayoutDescriptor {
        //         label: Some("Render Pipeline Layout"),
        //         bind_group_layouts: &[
        //             &texture_bind_group_layout,
        //             &camera_bind_group_layout,
        //             &light_bind_group_layout,
        //         ],
        //         push_constant_ranges: &[],
        //     },
        //     wgpu::ShaderModuleDescriptor {
        //     label: Some("Normal Shader"),
        //     source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        // });

        let light_render_pipeline = {
            let layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/light.wgsl").into()),
            };
            pipeline::create_render_pipeline(
                &device.device,
                &layout,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc()],
                wgpu::PrimitiveTopology::TriangleList,
                shader,
            )
        };

        let obj_model = resources::load_model("cube.obj", &device.device, &device.queue, &texture_bind_group_layout).await.unwrap();
        const SPACE_BETWEEN: f32 = 3.0;

        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
        
                let translation = cgmath::Vector3 { x, y: 0.0, z };
                let scale = cgmath::Vector3 { x: 1.0, y: 1.0, z: 1.0 };
                let rotation = cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(100.0));
        
                Transform3d {
                    translation, scale, rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Transform3d::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Self {
            window,
            device,
            config,
            size,
            render_pipeline,
            camera,
            projection,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            transform3ds: instances,
            instance_buffer,
            depth_texture,
            obj_model,
            light_uniform,
            light_buffer,
            light_bind_group_layout,
            light_bind_group,
            light_render_pipeline,
            mouse_pressed: false,
        }
    }

    pub fn window(&self) -> &MyWindow {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.device.surface.configure(&self.device.device, &self.config);
            self.depth_texture = texture::Texture::create_depth_texture(&self.device.device, &self.config, "depth_texture");
            self.projection.resize(new_size.width, new_size.height);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                },
            ..
            } => self.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel {delta, ..} => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput { 
                button: MouseButton::Left, 
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, delta: instant::Duration) {
       self.camera_controller.update_camera(&mut self.camera, delta);
       self.camera_uniform.update_view_proj(&self.camera, &self.projection);
       self.device.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));

       let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
       self.light_uniform.position = (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(60.0 * delta.as_secs_f32()))
            * old_position).into();
        self.device.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.device.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        { // In block to release mut borrow
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });


            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            
            render_pass.set_pipeline(&self.light_render_pipeline);
            render_pass.draw_light_model(
                &self.obj_model, 
                &self.camera_bind_group, 
                &self.light_bind_group
            );
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_model_instanced(
                &self.obj_model,
                0..self.transform3ds.len() as u32, 
                &self.camera_bind_group, 
                &self.light_bind_group
            );
        }

        self.device.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = MyWindow::new(800, 600, "wgpu engine", &event_loop);
    let mut state = State::new(window).await;
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta, },
                ..
            } => if state.mouse_pressed {
                state.camera_controller.process_mouse(delta.0, delta.1)
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.window.id() && !state.input(event) => {
                match event {
                    #[cfg(not(target_arch="wasm32"))]
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
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
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window.window.id() => {
                let now = instant::Instant::now();
                let delta = now - last_render_time;
                last_render_time = now;
                state.update(delta);
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!(" Test {:?}\n", e),
                }
            }
            Event::MainEventsCleared => {
                state.window.window.request_redraw();
            }
            _ => {}
        }
    });
}