#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate image;

mod teapot;

use glium::{
    draw_parameters::{BackfaceCullingMode, DepthTest},
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
    Depth, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
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

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                mat4 model_view = view * model;
                v_normal = transpose(inverse(mat3(model_view))) * normal;
                gl_Position = perspective * model_view * vec4(position, 1.0);
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

        let s: f32 = 0.002;

        let mut target_frame = display.draw();

        let perspective = {
            let (width, height) = target_frame.get_dimensions();
            let aspect_ration = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let znear = 0.1;
            let zfar = 1024.0;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ration, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };
        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

        let uniforms = uniform! {
            model: [
                [s,0.0,0.0,0.0],
                [0.0,s,0.0,0.0],
                [0.0,0.0,s,0.0],
                [0.0,0.0,0.6,1.0],
            ],
            u_light: [-1.0 , 0.4 , 0.9f32],
            perspective: perspective,
            view:view,
        };

        target_frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}