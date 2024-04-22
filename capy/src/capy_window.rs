use wgpu::{Adapter, Instance, Surface};
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

pub struct CapyWindow {
    pub width: u32,
    pub height: u32,
    frame_buffer_resized: bool,
    window_name: String,
    pub window: Window,
}

impl CapyWindow {
    pub fn new(w: u32, h: u32, name: &str, event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        Self {
            width: w,
            height: h,
            frame_buffer_resized: false,
            window_name: name.to_string(),
            window,
        }
    }

    pub fn was_window_resized(&self) -> bool {
        false
    }

    pub fn reset_window_resized_flag(&mut self) {
        self.frame_buffer_resized = false;
    }

    pub fn get_winit_window(&self) -> &Window {
        &self.window
    }


    pub fn create_surface(&self, instance: &Instance) -> wgpu::Surface {
        let surface = unsafe { 
            instance.create_surface(&self.window) 
        };
        surface.unwrap()
    }
}
