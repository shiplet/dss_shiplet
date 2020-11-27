use glium::{glutin, VertexBuffer, Surface, Program, Display};
use glium::backend::glutin::glutin::dpi::PhysicalSize;
use glium::backend::glutin::glutin::event_loop::{EventLoop, ControlFlow};
use crate::Vertex;
use glium::backend::glutin::glutin::event::{Event, VirtualKeyCode};
use std::io::Cursor;
use glium::index::NoIndices;
use glium::texture::{RawImage2d, Texture2dDataSource};

pub struct Screen<'a> {
    pub display: Display,
    pub program: Option<Program>,
    pub indices: NoIndices,
    pub horizontal: f32,
    pub vertical: f32,
    pub vertex_buffers: Vec<VertexBuffer<Vertex>>,
    pub texture: Option<RawImage2d<'a, u16>>
}

impl<'a> Screen<'a> {
    pub fn new(width: i32, height: i32, event_loop: &EventLoop<()>) -> Screen<'a> {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Disney Streaming Services Homework")
            .with_inner_size(PhysicalSize::new(width, height));
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, event_loop).unwrap();
        Screen {
            display,
            program: None,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            horizontal: 0.0,
            vertical: 0.0,
            vertex_buffers: Vec::new(),
            texture: None
        }
    }

    pub fn use_default_shaders(&mut self) {
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
            // color = texture(tex, v_tex_coords);
            color = vec4(0.5, 0.0, 0.25, 1.0);
        }
        "#;

        self.program = Some(glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap());
    }

    pub fn add_shape(&mut self, v: &[Vertex]) {
        let vertex_buffer = glium::VertexBuffer::new(&self.display, v).unwrap();
        self.vertex_buffers.push(vertex_buffer);
    }

    pub fn add_shapes(&mut self, shapes: Vec<Vec<Vertex>>) {
        for n in shapes {
            self.add_shape(&n);
        }
    }

    pub fn add_texture(&mut self) {
        let img = image::load(Cursor::new(&include_bytes!("./right_stuff.jpg")[..]),
                              image::ImageFormat::Jpeg).unwrap().to_rgba16();
        let image_dimensions = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
        self.texture = Some(img.into_raw())
        // let tex = glium::texture::Texture2d::new(&self.display, img).unwrap();
        // self.texture = Some(tex);
    }

    pub fn render(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow) {
        let p = match &self.program {
            Some(pg) => pg,
            None => panic!("must specify shaders - try calling use_default_shaders before running loop")
        };


        let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [ self.horizontal , self.vertical, 0.0, 1.0f32],
                ],
                // tex: &tex,
            };
        let mut target = self.display.draw();
        target.clear_color(0.0,0.0,0.0,1.0);
        for buffer in self.vertex_buffers.iter() {
            target.draw(buffer, &self.indices, &p, &uniforms,
                        &Default::default()).unwrap();
        }
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(166_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            Event::WindowEvent {event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                glutin::event::WindowEvent::KeyboardInput {  input, .. } => match input.virtual_keycode {
                    Some(vk) => match vk {
                        VirtualKeyCode::Up => { // Arrow up
                            self.vertical += 0.02;
                            // println!("position: {}, {}", horizontal, vertical)
                        },
                        VirtualKeyCode::Down => { // Arrow down
                            self.vertical -= 0.02;
                            // println!("position: {}, {}", horizontal, vertical)
                        },
                        VirtualKeyCode::Right => { // Arrow right
                            self.horizontal += 0.02;
                            // println!("position: {}, {}", horizontal, vertical)
                        },
                        VirtualKeyCode::Left => { // Arrow left
                            self.horizontal -= 0.02;
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

    }
}



