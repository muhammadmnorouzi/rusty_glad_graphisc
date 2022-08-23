#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;

use glium::{
    glutin::{
        dpi::LogicalSize,
        event::{self, Event, StartCause},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    uniforms::EmptyUniforms,
    Program, Surface, VertexBuffer,
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

    let indices = NoIndices(PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 matrix;
            out vec2 out_pos;

            void main() {
                out_pos = position;
                gl_Position = matrix * vec4(position , 0.0 , 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 140

            in vec2 out_pos;
            out vec4 color;

            void main() {
                // color = vec4(1.0,1.0,0.0,1.0);
                vec2 pos = out_pos;
                pos.x += 0.3;
                pos.y -= 0.1;
                color = vec4(0.0 ,pos,1.0);
            }
        "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("failed to create program!");

    let shape = vec![
        Vertex::create(-0.5, -0.5),
        Vertex::create(0.0, 0.0),
        Vertex::create(0.5, -0.25),
    ];

    let vertex_buffer =
        VertexBuffer::new(&display, &shape).expect("failed to create vertex buffer!");

    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                event::WindowEvent::Focused(focused) => {
                    if focused {
                        t += 0.05;
                    }
                }
                _ => return,
            },
            Event::NewEvents(reason) => match reason {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        t += 0.005;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0,0.0,0.0,0.0],
                [0.0,1.0,0.0,0.0],
                [0.0,0.0,1.0,0.0],
                [t,0.0,0.0,1.0f32],
            ]
        };

        let mut target_frame = display.draw();
        target_frame.clear_color(0.9, 0.6, 0.3, 1.0);

        target_frame
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                // &EmptyUniforms,
                &uniforms,
                &Default::default(),
            )
            .expect("failed to draw program!");

        target_frame.finish().expect("failed to draw on screen");

        let next_frame_time = Instant::now() + Duration::from_nanos(17_000_000);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);
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
