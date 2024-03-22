use wgpu::{Adapter, Instance, Surface};
//use winit::window::Window;

use crate::my_window::MyWindow;

pub struct Device {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Device {
    pub async fn new(window: &MyWindow) -> Self {
        let instance = create_instance();
        let surface = window.create_surface(&instance);
        let adapter = request_adapter(&instance, &surface).await;
        let (device, queue) = request_device(&adapter).await;
        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }
}

fn create_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    })
}

// fn create_surface(instance: &Instance, window: &Window) -> wgpu::Surface {
//     let surface = unsafe { 
//         instance.create_surface(&window) 
//     };
//     surface.unwrap()
// }

async fn request_adapter(instance: &Instance, surface: &Surface) -> wgpu::Adapter {
    instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap()
}

async fn request_device(adapter: &Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
        }, 
        None,
    ).await.unwrap()
}