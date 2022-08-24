#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate image;

mod teapot;

use glium::{
    DrawParameters , Depth ,
    draw_parameters::DepthTest,
    glutin::{
        dpi::LogicalSize,
        event::{self, Event, StartCause},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    implement_vertex,
    index::{NoIndices, PrimitiveType},
    texture::{RawImage2d, SrgbTexture2d},
    uniforms::EmptyUniforms,
    IndexBuffer, Program, Surface, VertexBuffer,
};
use std::{
    fs,
    io::Cursor,
    path::Path,
    time::{Duration, Instant},
    vec::Vec,
};

pub fn main() {
    let mut event_loop = EventLoop::new();
    let context_builder = ContextBuilder::new().with_depth_buffer(24);

    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(720, 480))
        .with_title("Vectoria");
    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("failed to create Display object");

    let positions = VertexBuffer::new(&display, &teapot::VERTICES)
        .expect("failed to create new VertexBuffer of VERTICIES.");

    let normals = VertexBuffer::new(&display, &teapot::NORMALS).expect("creating normals failed!");

    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES)
        .expect("failed to create indices!");

    let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            uniform mat4 matrix;

            void main() {
                v_normal = transpose(inverse(mat3(matrix))) * normal;
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 140

            in vec3 v_normal;
            out vec4 color;
            uniform vec3 u_light;

            void main() {
                float brightness = dot(normalize(v_normal),normalize(u_light));
                vec3 dark_color = vec3(0.5 , 0.0 , 0.0);
                vec3 regular_color = vec3(1.0 , 0.0 , 0.0);
                color = vec4(mix(dark_color , regular_color , brightness) , 1.0 );
            }
        "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("failed to create program!");

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(17_000_000);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let s: f32 = 0.008;

        let uniforms = uniform! {
            matrix: [
                [s,0.0,0.0,0.0],
                [0.0,s,0.0,0.0],
                [0.0,0.0,s,0.0],
                [0.0,0.0,0.0,1.0],
            ],
            u_light: [-1.0 , 0.4 , 0.9f32]
        };

        let mut target_frame = display.draw();
        target_frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let params = DrawParameters{
            depth: Depth{
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        target_frame
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                // &EmptyUniforms,
                &uniforms,
                &params,
            )
            .expect("failed to draw program!");

        target_frame.finish().expect("failed to draw on screen");
    });
}
