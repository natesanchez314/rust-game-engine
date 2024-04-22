use winit::{dpi::PhysicalSize, event::{DeviceEvent, Event}, event_loop::{ControlFlow, EventLoop}};

use crate::capy_window::{self, CapyWindow};

pub struct App {
}

impl App {
    pub fn new() -> Self {
        
        Self {
        }
    }

    pub fn run(&mut self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = CapyWindow::new(800, 600, "Capy Engine v0.1", &event_loop);

        let mut last_render_time = instant::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            // Pass event to event dispatcher/add to event queue
        });
    }
}