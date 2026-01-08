use anyhow::Result;
use renderer::{Renderer, color::Color};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Scene {
    renderer: Renderer,
    event_loop: EventLoop<()>,
}

impl Scene {
    pub fn default() -> Result<Self> {
        let event_loop = EventLoop::new();
        let renderer = Renderer::default(&event_loop)?;
    
        Ok(Self { renderer, event_loop })
    }

    pub fn fill(&mut self, color: &Color) {
        self.renderer.fill(color);
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn draw_sprite(&mut self) -> Result<()> {
        
        Ok(())
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            self.renderer.handle_event(event, control_flow);
        })
    }
}