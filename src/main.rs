#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::backend::glutin::glutin::event::{Event};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 120

        attribute vec2 position;

        uniform float t;

        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 120

        uniform float c;
        vec3 nc;

        void main() {
            nc = vec3(1.0, 0.25, 0.37);
            nc.xz += c;
            nc.y -= c;
            gl_FragColor = vec4(nc, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut c: f32 = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.02;
        if t > 1.5 {
            t = -1.5;
        }

        c += 0.00002;
        if c >= 1.0 {
            c = 0.0;
        }

        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {t: t, c: c},
                    &Default::default()).unwrap();
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_6667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent {event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    ()
                },
                _ => ()
            },
            _ => (),
        }
    });


}
