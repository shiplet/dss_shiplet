#[macro_use]
extern crate glium;
extern crate image;

use glium::{glutin, Surface};
use glium::backend::glutin::glutin::event::{Event};
use glium::backend::glutin::glutin::dpi::PhysicalSize;
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Disney Streaming Services Homework")
        .with_inner_size(PhysicalSize::new(1440, 900));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [-0.25,  0.25], tex_coords: [0.0, 0.99]};
    let vertex2 = Vertex { position: [-0.25, -0.25], tex_coords: [0.0, 0.0]};
    let vertex3 = Vertex { position: [ 0.25, -0.25], tex_coords: [0.99, 0.0]};
    let vertex4 = Vertex { position: [-0.25,  0.25], tex_coords: [0.0, 0.99]};
    let vertex5 = Vertex { position: [ 0.25,  0.25], tex_coords: [0.99, 0.99]};
    let vertex6 = Vertex { position: [ 0.25, -0.25], tex_coords: [0.99, 0.0]};
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let img = image::load(Cursor::new(&include_bytes!("./right_stuff.jpg")[..]),
    image::ImageFormat::Jpeg).unwrap().to_rgba16();
    let image_dimensions = img.dimensions();
    let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
    let tex = glium::texture::Texture2d::new(&display, img).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut vertical:f32 = 0.0;
    let mut horizontal:f32 = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ horizontal , vertical, 0.0, 1.0f32],
            ],
            tex: &tex,
        };
        let mut target = display.draw();
        target.clear_color(0.0,0.0,0.0,1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_6667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent {event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return
                },
                glutin::event::WindowEvent::KeyboardInput {  input, .. } => match input.scancode {
                    126 => { // Arrow up
                        vertical += 0.02;
                        println!("position: {}, {}", horizontal, vertical)
                    },
                    125 => { // Arrow down
                        vertical -= 0.02;
                        println!("position: {}, {}", horizontal, vertical)
                    },
                    124 => { // Arrow right
                        horizontal += 0.02;
                        println!("position: {}, {}", horizontal, vertical)
                    },
                    123 => { // Arrow left
                        horizontal -= 0.02;
                        println!("position: {}, {}", horizontal, vertical)
                    },
                    _ => ()
                }
                _ => ()
            },
            _ => (),
        }
    });


}
