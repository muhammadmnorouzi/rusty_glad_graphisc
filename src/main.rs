#![allow(unused)]
#![allow(dead_code)]

#[macro_use]
extern crate glium;

use glium::glutin::{
    dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

fn main() {
    let mut event_loop = EventLoop::new();
    let context_builder = ContextBuilder::new();

    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(620, 410))
        .with_title("Vectoria");

    let display = glium::Display::new(window_builder, context_builder, &event_loop)
        .expect("failed to create Display object");

        
}
