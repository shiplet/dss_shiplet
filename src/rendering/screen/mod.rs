use glium::{glutin, VertexBuffer, Surface};
use glium::backend::glutin::glutin::dpi::PhysicalSize;
use glium::backend::glutin::glutin::event_loop::EventLoop;
use crate::Vertex;
use glium::backend::glutin::glutin::event::{Event, VirtualKeyCode};
use std::io::Cursor;
use std::marker::PhantomData;

pub struct Screen<'a> {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub vertex_buffers: Vec<VertexBuffer<Vertex>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Screen<'a> {
    pub fn add_shape(&mut self, v: &Vec<Vertex>) {
        let vertex_buffer = glium::VertexBuffer::new(&self.display, v).unwrap();
        self.vertex_buffers.push(vertex_buffer);
    }
    pub fn new(width: i32, height: i32) -> Screen<'a> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Disney Streaming Services Homework")
            .with_inner_size(PhysicalSize::new(width, height));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        Screen {
            event_loop,
            display,
            vertex_buffers: Vec::new(),
            phantom: PhantomData::default()
        }
    }
    pub fn run(&self) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let img = image::load(Cursor::new(&include_bytes!("./right_stuff.jpg")[..]),
                              image::ImageFormat::Jpeg).unwrap().to_rgba16();
        let image_dimensions = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
        let tex = glium::texture::Texture2d::new(&self.display, img).unwrap();
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

        let program = glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut horizontal: f32 = 0.0;
        let mut vertical: f32 = 0.0;
        self.event_loop.run(|ev, _, control_flow| {
            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ horizontal , vertical, 0.0, 1.0f32],
                ],
                tex: &tex,
            };
            let mut target = self.display.draw();
            target.clear_color(0.0,0.0,0.0,1.0);
            for buffer in self.vertex_buffers.iter() {
                target.draw(buffer, &indices, &program, &uniforms,
                            &Default::default()).unwrap();
            }
            target.finish().unwrap();
            let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_6667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            match ev {
                Event::WindowEvent {event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        ()
                    },
                    glutin::event::WindowEvent::KeyboardInput {  input, .. } => match input.virtual_keycode {
                        Some(vk) => match vk {
                            VirtualKeyCode::Up => { // Arrow up
                                vertical += 0.02;
                                // println!("position: {}, {}", horizontal, vertical)
                            },
                            VirtualKeyCode::Down => { // Arrow down
                                vertical -= 0.02;
                                // println!("position: {}, {}", horizontal, vertical)
                            },
                            VirtualKeyCode::Right => { // Arrow right
                                horizontal += 0.02;
                                // println!("position: {}, {}", horizontal, vertical)
                            },
                            VirtualKeyCode::Left => { // Arrow left
                                horizontal -= 0.02;
                                // println!("position: {}, {}", horizontal, vertical)
                            },
                            _ => ()
                        },
                        _ => (),
                    }
                    _ => ()
                },
                _ => (),
            }
        });
    }
}



