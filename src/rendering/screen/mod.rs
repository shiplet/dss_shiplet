use glium::backend::glutin::glutin::dpi::PhysicalSize;
use glium::backend::glutin::glutin::event::{Event, VirtualKeyCode};
use glium::backend::glutin::glutin::event_loop::{EventLoop, ControlFlow};
use glium::index::NoIndices;
use glium::texture::{RawImage2d, Texture2dDataSource};
use glium::{glutin, VertexBuffer, Surface, Program, Display};
use glium_glyph::glyph_brush::{rusttype::Font, Section};
use glium_glyph::GlyphBrush;
use std::cmp::{min, max};
use std::io::Cursor;
use std::time::{Duration, Instant};

use crate::{Vertex, DEBUG};
use crate::rendering::shapes::{VertexData, Row};
use glium_glyph::glyph_brush::rusttype::Scale;
use crate::types::Container;

#[derive(Debug)]
pub struct VertexBufferContainer {
	pub buffer: VertexBuffer<VertexData>,
	pub self_location: [f32; 2],
	pub translate_dist: [f32; 2],
}

pub struct ActiveLocation {
	debounce: Duration,
	last_tick: Instant,
	x: f32,
	y: f32,
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

pub struct Screen<'a> {
	pub active_limit_x: f32,
	pub active_limit_y: f32,
	pub active_location: ActiveLocation,
	pub active_rows: Vec<Container>,
	pub active_rows_count: f32,
	pub current_row_positions: Vec<f32>,
	pub display: Display,
	pub horizontal: f32,
	pub indices: NoIndices,
	pub program: Option<Program>,
	pub text_renderer: GlyphBrush<'a,'a>,
	pub texture: Option<RawImage2d<'a, u16>>,
	pub vertex_buffers: Vec<VertexBufferContainer>,
	pub vertical: f32,
}

impl<'a> Screen<'a> {
	pub fn new(width: i32, height: i32, event_loop: &EventLoop<()>) -> Screen<'a> {
		let wb = glutin::window::WindowBuilder::new()
			.with_title("Disney Streaming Services Homework")
			.with_inner_size(PhysicalSize::new(width, height));
		let cb = glutin::ContextBuilder::new();
		let display = glium::Display::new(wb, cb, event_loop).unwrap();
		let fonts = vec![Font::from_bytes(include_bytes!("./fonts/NunitoSans-SemiBold.ttf")).unwrap()];
		let text_renderer = GlyphBrush::new(&display, fonts);
		Screen {
			active_limit_x: 0.0,
			active_limit_y: 0.0,
			active_location: ActiveLocation{ x: 0.0, y: 0.0, last_tick: Instant::now(), debounce: Duration::from_millis(200) },
			active_rows: Vec::new(),
			active_rows_count: 0.0,
			current_row_positions: Vec::new(),
			display,
			horizontal: 0.0,
			indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			program: None,
			text_renderer,
			texture: None,
			vertex_buffers: Vec::new(),
			vertical: 0.0,
		}
	}

	pub fn set_active_rows(&mut self, rows: Vec<Container>) {
		self.active_rows = rows;
		self.active_rows_count = self.active_rows.len() as f32;
		self.current_row_positions = vec![0.0 as f32; self.active_rows.len() as usize];
		self.active_limit_x = (((1.0_f32 / 3.75_f32) * 2.0_f32) * 10.0).floor();
		self.active_limit_y = ((1.0_f32 / 3.0_f32) * 10.0).floor();
	}

	pub fn use_default_shaders(&mut self) {
		let vertex_shader_src = r#"
		#version 140

		in vec2 position;
		in vec2 tex_coords;
		out vec2 v_tex_coords;

		uniform float scale;
		uniform mat4 matrix;
		uniform vec2 active_location;
		uniform vec2 self_location;
		uniform vec2 td;

		void main() {
			vec2 pos;
			if (self_location == active_location) {
				pos = position;
				pos += vec2(td.x, td.y);
				pos = pos * scale;
				pos -= vec2(td.x, td.y);
			} else {
				pos = position;
			}
			v_tex_coords = tex_coords;
			gl_Position = matrix * vec4(pos, 0.0, 1.0);
		}
		"#;

		let fragment_shader_src = r#"
		#version 140

		in vec2 v_tex_coords;
		out vec4 color;

		uniform sampler2D tex;
		uniform vec2 active_location;
		uniform vec2 self_location;
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
			self_location: v[0].self_location.unwrap(), // only need the first one since it's the left-most x-coordinate
			translate_dist: v[0].translate_dist.unwrap(), // ^^ ditto here
		};
		self.vertex_buffers.push(vbc);
	}

	pub fn add_row(&mut self, row: Row) {
		for n in row.tiles.unwrap() {
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
		let program = match &self.program {
			Some(pg) => pg,
			None => panic!("must specify shaders - try calling use_default_shaders before running loop")
		};

		let mut target = self.display.draw();
		target.clear_color(0.0, 0.0, 0.0, 1.0);
		let active_location = self.active_location.to_vec();
		let _x_trans_overflow = (active_location[0] + 1.0) - self.active_limit_x;
		let mtx = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[self.horizontal, self.vertical, 0.0, 1.0f32], // translation components
		];

		for buffer in self.vertex_buffers.iter() {
			let _xt: f32 = 0.0;
			// let mut yt: f32 = 0.0;
			let self_location = buffer.self_location;
			let translate_distance = buffer.translate_dist;
			if DEBUG {
				print!("\u{001b}[1000D");
				print!("\u{001b}[{}A", self.active_rows_count as usize);
				print!("{}", format!("row: {:.3?}\n", self_location[1]).repeat(self.active_rows_count as usize));
			}

			let uniforms = uniform! {
				active_location: active_location,
				matrix: mtx,
				scale: 1.25 as f32,
				self_location: self_location,
				td: translate_distance,
			};
			target.draw(&buffer.buffer, &self.indices, &program, &uniforms,
						&Default::default()).unwrap();
		}

		let screen_dims = self.display.get_framebuffer_dimensions();

		self.text_renderer.queue(Section{
			text: "This is a test",
			bounds: (screen_dims.0 as f32, screen_dims.1 as f32 / 2.0),
			color: [1.0, 1.0, 1.0, 1.0],
			scale: Scale::uniform(24.0),
			..Section::default()
		});

		self.text_renderer.draw_queued(&self.display, &mut target);
		target.finish().unwrap();
		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(166_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		if let Event::WindowEvent { event, .. } = ev {
			match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
				}
				glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
					Some(vk) => match vk {
						VirtualKeyCode::Up => { self.active_location.move_up(); }
						VirtualKeyCode::Down => { self.active_location.move_down(); }
						VirtualKeyCode::Left => { self.active_location.move_left(); }
						VirtualKeyCode::Right => { self.active_location.move_right(); }
						VirtualKeyCode::W => { self.vertical -= 0.1; }
						VirtualKeyCode::A => { self.horizontal += 0.1; }
						VirtualKeyCode::S => { self.vertical += 0.1; }
						VirtualKeyCode::D => { self.horizontal -= 0.1; }
						_ => ()
					},
					_ => (),
				}
				_ => ()
			}
		}
	}
}



