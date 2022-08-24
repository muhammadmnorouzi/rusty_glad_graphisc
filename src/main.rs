#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate image;

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
    texture::{RawImage2d, SrgbTexture2d},
    uniforms::EmptyUniforms,
    Program, Surface, VertexBuffer,
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
            in vec2 tex_coords;

            out vec2 v_tex_coords;
            out vec2 out_pos;

            uniform mat4 matrix;

            void main() {
                v_tex_coords = tex_coords;
                out_pos = position;
                gl_Position = matrix * vec4(position , 0.0 , 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;

            void main() {
                color = texture(tex , v_tex_coords);
            }
        "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("failed to create program!");

    let shape = vec![
        Vertex::create([-0.5, -0.5], [0.0, 0.0]),
        Vertex::create([0.0, 0.0], [0.0, 1.0]),
        Vertex::create([0.5, -0.25], [1.0, 0.0]),
    ];

    let vertex_buffer =
        VertexBuffer::new(&display, &shape).expect("failed to create vertex buffer!");

    let image_path = Path::new(".")
        .join("src")
        .join("resources")
        .join("opengl.jpg");
    println!("file path : {:?}", image_path);
    let image_content: Vec<u8> = fs::read(image_path).expect("failed to find image!");

    let image = image::load(
        // Cursor::new(&include_bytes!("../resources/duck.png")),
        Cursor::new(&image_content),
        image::ImageFormat::Jpeg,
    )
    .expect("failed to load image!")
    .to_rgba8();

    let image_dimensions = image.dimensions();
    println!("image dimensions : {:?}", image_dimensions);

    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = SrgbTexture2d::new(&display, image).expect("failed to create texture!");

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
            ],
            tex: &texture
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
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    fn create(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}
