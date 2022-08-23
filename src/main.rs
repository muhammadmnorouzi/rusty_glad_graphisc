#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;

use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::{NonIndices, PrimitiveType},
    Surface, VertexBuffer,
};
use std::time::{Duration, Instant};

pub fn main() {
    let mut event_loop = EventLoop::new();
    let context_builder = ContextBuilder::new();

    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(620, 410))
        .with_title("Vectoria");

    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("failed to create Display object");

    let shape = vec![
        Vertex::create(-0.5, -0.5),
        Vertex::create(0.0, 0.0),
        Vertex::create(0.5, -0.25),
    ];

    let vertex_buffer =
        VertexBuffer::new(&display, &shape).expect("failed to create vertex buffer!");

    let indices = NonIndices(PrimitiveType::TrianglesList);

    event_loop.run(move |event, _, control_flow| {
        let mut target_frame = display.draw();
        target_frame.clear_color(0.5, 0.0, 1.0, 0.5);
        target_frame.finish().expect("failed to draw on screen");

        let next_frame_time = Instant::now() + Duration::from_nanos(17_000_000);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    fn create(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}
