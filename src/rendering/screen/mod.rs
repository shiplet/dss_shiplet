use glium::{glutin, VertexBuffer, Surface, Program, Display};
use glium::backend::glutin::glutin::dpi::PhysicalSize;
use glium::backend::glutin::glutin::event_loop::{EventLoop, ControlFlow};
use glium::backend::glutin::glutin::event::{Event, VirtualKeyCode};
use std::io::Cursor;
use glium::index::NoIndices;
use glium::texture::{RawImage2d, Texture2dDataSource};
use std::cmp::{min, max};
use std::time::{Duration, Instant};

use crate::Vertex;
use crate::rendering::shapes::VertexData;

pub struct Screen<'a> {
	pub active_location: ActiveLocation,
	pub display: Display,
	pub program: Option<Program>,
	pub indices: NoIndices,
	pub horizontal: f32,
	pub vertical: f32,
	pub vertex_buffers: Vec<VertexBufferContainer>,
	pub texture: Option<RawImage2d<'a, u16>>,
}

pub struct ActiveLocation {
	x: f32,
	y: f32,
	last_tick: Instant,
	debounce: Duration
}

impl ActiveLocation {
	pub fn to_vec(&self) -> [f32; 2] { [self.x, self.y] }
	pub fn move_up(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.y = max(0, self.y as i32 - 1) as f32;
			self.last_tick = Instant::now();
		}
	}
	pub fn move_down(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.y = min(4, self.y as i32 + 1) as f32;
			self.last_tick = Instant::now();
		}
	}
	pub fn move_left(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.x = max(0, self.x as i32 - 1) as f32;
			self.last_tick = Instant::now();
		}
	}
	pub fn move_right(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.x = min(15, self.x as i32 + 1) as f32;
			self.last_tick = Instant::now();
		}
	}
}

#[derive(Debug)]
pub struct VertexBufferContainer {
	pub buffer: VertexBuffer<VertexData>,
	pub self_location: [f32; 2],
	pub translate_dist: [f32; 2],
}

impl<'a> Screen<'a> {
	pub fn new(width: i32, height: i32, event_loop: &EventLoop<()>) -> Screen<'a> {
		let wb = glutin::window::WindowBuilder::new()
			.with_title("Disney Streaming Services Homework")
			.with_inner_size(PhysicalSize::new(width, height));
		let cb = glutin::ContextBuilder::new();
		let display = glium::Display::new(wb, cb, event_loop).unwrap();
		Screen {
			active_location: ActiveLocation{ x: 0.0, y: 0.0, last_tick: Instant::now(), debounce: Duration::from_millis(200) },
			display,
			program: None,
			indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			horizontal: 0.0,
			vertical: 0.0,
			vertex_buffers: Vec::new(),
			texture: None,
		}
	}

	pub fn use_default_shaders(&mut self) {
		let vertex_shader_src = r#"
		#version 140

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;

		uniform mat4 matrix;
		uniform vec2 self_location;
		uniform vec2 active_location;
		uniform vec2 td;
		uniform float scale;

		void main() {
			vec2 pos;
			if (self_location == active_location) {
				pos = position;
				pos += vec2(td.x, td.y);
				pos = pos * scale;
				pos -= vec2(td.x, td.y);
				gl_Position = matrix * vec4(pos, 0.0, 1.0);
			} else {
				pos = position;
				gl_Position = vec4(pos, 0.0, 1.0);
			}
			v_tex_coords = tex_coords;
		}
		"#;

		let fragment_shader_src = r#"
		#version 140

		in vec2 v_tex_coords;
		out vec4 color;

		uniform sampler2D tex;
		uniform vec2 self_location;
		uniform vec2 active_location;
		vec2 pos;

		void main() {
			if (self_location == active_location) {
				color = vec4(0.5, 0.25, 0.25, 1.0);
			} else {
				color = vec4(0.5, 0.0, 0.25, 1.0);
			}
		}
		"#;

		self.program = Some(glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap());
	}

	pub fn add_shape(&mut self, v: &[Vertex]) {
		let mut v_data = Vec::new();
		for vtx in v.iter() {
			v_data.push(vtx.data);
		}
		let vertex_buffer = glium::VertexBuffer::new(&self.display, &v_data).unwrap();
		let vbc = VertexBufferContainer{
			buffer: vertex_buffer,
			self_location: v[0].self_location,
			translate_dist: v[0].translate_dist,
		};
		self.vertex_buffers.push(vbc);
	}

	pub fn add_shapes(&mut self, shapes: Vec<Vec<Vertex>>) {
		for n in shapes {
			self.add_shape(&n);
		}
	}

	#[allow(dead_code)]
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

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		let active_location = self.active_location.to_vec();
		for buffer in self.vertex_buffers.iter() {
			let self_location = buffer.self_location;
			let translate_distance = buffer.translate_dist;
			let uniforms = uniform! {
				matrix: [
					[1.0, 0.0, 0.0, 0.0],
					[0.0, 1.0, 0.0, 0.0],
					[0.0, 0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0, 1.0f32], // translation components
				],
				self_location: self_location,
				active_location: active_location,
				scale: 1.25 as f32,
				td: translate_distance,
			};
			target.draw(&buffer.buffer, &self.indices, &p, &uniforms,
						&Default::default()).unwrap();
		}
		target.finish().unwrap();
		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(166_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		match ev {
			Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
				}
				glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
					Some(vk) => match vk {
						VirtualKeyCode::Up => { self.active_location.move_up(); }
						VirtualKeyCode::Down => { self.active_location.move_down(); }
						VirtualKeyCode::Left => { self.active_location.move_left(); }
						VirtualKeyCode::Right => { self.active_location.move_right(); }
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



