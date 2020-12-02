use glium::backend::glutin::glutin::dpi::PhysicalSize;
use glium::backend::glutin::glutin::event::{Event, VirtualKeyCode};
use glium::backend::glutin::glutin::event_loop::{EventLoop, ControlFlow};
use glium::index::NoIndices;
use glium::{glutin, VertexBuffer, Surface, Program, Display};
use glium_glyph::GlyphBrush;
use glium_glyph::glyph_brush::rusttype::Scale;
use glium_glyph::glyph_brush::{rusttype::Font, Section};
use std::cmp::{min, max};
use std::io::{Cursor, stdout, Write};
use std::time::{Duration, Instant};

use crate::rendering::shapes::{VertexData, Row};
use crate::data::types::Container;
use crate::Vertex;

use bytes::Buf;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Screen<'a> {
	pub active_location: ActiveLocation,
	pub active_scale_ratio: f32,
	pub current_row_positions: Vec<f32>,
	pub display: Display,
	pub global_position_cache: HashMap<String, f32>,
	pub height: i32,
	pub horizontal: f32,
	pub indices: NoIndices,
	pub program: Option<Program>,
	pub row_titles: Vec<ScreenRowTitle>,
	pub rows: Vec<Container>,
	pub rows_count: f32,
	pub text_renderer: GlyphBrush<'a,'a>,
	pub texture: Option<glium::texture::SrgbTexture2d>,
	pub texture_cache: HashMap<String, glium::texture::SrgbTexture2d>,
	pub vertex_buffers: Vec<VertexBufferContainer>,
	pub vertical: f32,
	pub width: i32
}

impl<'a> Screen<'a> {
	pub fn new(width: i32, height: i32, event_loop: &EventLoop<()>) -> Screen<'a> {
		let wb = glutin::window::WindowBuilder::new()
			.with_title("Shiplet DSS Homework")
			.with_inner_size(PhysicalSize::new(width, height));
		let cb = glutin::ContextBuilder::new();
		let display = glium::Display::new(wb, cb, event_loop).unwrap();
		let fonts = vec![Font::from_bytes(include_bytes!("./fonts/NunitoSans-SemiBold.ttf")).unwrap()];
		let text_renderer = GlyphBrush::new(&display, fonts);
		let active_location = ActiveLocation{
			debounce: Duration::from_millis(200),
			last_tick: Instant::now(),
			x: 0,
			y: 0,
			x_limit: 0,
			y_limit: 0,
			virtual_x: 0,
			virtual_y: 0,
			virtual_x_limit: ((2.0 / 0.375) as f32).floor() as i32,
			virtual_y_limit: ((2.0 / 0.625) as f32).floor() as i32,
			x_cache: HashMap::new()
		};
		Screen {
			active_location,
			active_scale_ratio: 1.45,
			current_row_positions: Vec::new(),
			display,
			global_position_cache: HashMap::new(),
			horizontal: 0.0,
			indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			program: None,
			row_titles: Vec::new(),
			rows: Vec::new(),
			rows_count: 0.0,
			text_renderer,
			texture: None,
			texture_cache: HashMap::new(),
			vertex_buffers: Vec::new(),
			vertical: 0.0,
			width,
			height
		}
	}

	pub fn set_active_rows(&mut self, rows: Vec<Container>) {
		self.rows = rows;
		self.rows_count = self.rows.len() as f32;
		self.active_location.set_max_rows(self.rows_count as i32);
		self.current_row_positions = vec![0.0 as f32; self.rows.len() as usize];
	}

	pub fn use_default_tile_shaders(&mut self) {
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

		void main() {
			color = texture(tex, v_tex_coords);
		}
		"#;

		self.program = Some(glium::Program::from_source(&self.display, vertex_shader_src, fragment_shader_src, None).unwrap());
	}

	pub fn add_tile(&mut self, v: &[Vertex]) {
		let mut v_data = Vec::new();
		for vtx in v.iter() {
			v_data.push(vtx.data);
		}
		let vertex_buffer = glium::VertexBuffer::new(&self.display, &v_data).unwrap();
		let texture = v[0].texture.clone().unwrap();
		let vbc = VertexBufferContainer{
			buffer: vertex_buffer,
			self_location: v[0].self_location.unwrap(), // only need the first one since it's the leftmost x-coordinate
			tst_distance: v[0].tst_distance.unwrap(), 	// ^^ ditto here
			texture_bytes: texture.texture_bytes,		// ^^ ditto again
			texture_id: texture.texture_id
		};
		self.vertex_buffers.push(vbc);
	}

	pub fn add_row(&mut self, row: Row) {
		self.active_location.set_max_tiles(row.tiles.as_ref().unwrap().len() as i32);
		self.row_titles.push(ScreenRowTitle {
			title: row.title,
			pos: row.title_pos
		});
		for n in row.tiles.unwrap() {
			self.add_tile(&n);
		}
	}

	fn get_or_create_texture(texture_map: &'a mut HashMap<String, glium::texture::SrgbTexture2d>, display: &Display, tex_id: String, tex_bytes: &bytes::Bytes) -> &'a glium::texture::SrgbTexture2d {
        match texture_map.entry(tex_id.clone()) {
            Entry::Occupied(t) => t.into_mut(),
			Entry::Vacant(m) => {
				let img = image::load_from_memory(tex_bytes.bytes());
				match img {
					Ok(i) => {
						let img = &i.to_rgba16();
						let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.as_raw(), img.dimensions());
						let t = glium::texture::SrgbTexture2d::new(display, img).unwrap();
						m.insert(t)
					},
					Err(_) => {
						let img = image::load(Cursor::new(&include_bytes!("./images/disney_bg.png")[..]),
											  image::ImageFormat::Png).unwrap().to_rgba16();
						let img = glium::texture::RawImage2d::from_raw_rgba_reversed(img.as_raw(), img.dimensions());
						let t = glium::texture::SrgbTexture2d::new(display, img).unwrap();
						m.insert(t)
					}
				}
			}
		};
        texture_map.get(tex_id.as_str()).unwrap()
	}

	pub fn render(&mut self, ev: &Event<()>, control_flow: &mut ControlFlow, texture_cache: &mut HashMap<String, glium::texture::SrgbTexture2d>) {
		let program = match self.program.as_mut() {
			Some(pg) => pg,
			None => panic!("must specify shaders - try calling use_default_shaders before running loop")
		};

		let mut target = self.display.draw();
		target.clear_color(0.00625, 0.00625, 0.00625, 1.0);
		let active_location = self.active_location.to_vec();


		let vertical = (self.active_location.y - self.active_location.virtual_y) as f32 * 0.625;
		for buffer in self.vertex_buffers.iter() {
			let self_location = buffer.self_location;
			let tst_distance = buffer.tst_distance;
			if self_location[1] == active_location[1] {
				let horizontal = (self.active_location.x - self.active_location.virtual_x) as f32 * -0.375;
				self.global_position_cache.insert(self_location[1].to_string(), horizontal);
			}
			let horizontal = self.global_position_cache.entry(self_location[1].to_string()).or_insert(0.0).to_owned();
			let mtx = [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[horizontal, vertical, 0.0, 1.0f32], // translation components
			];

            let img = Screen::get_or_create_texture(texture_cache, &self.display, buffer.texture_id.clone(), &buffer.texture_bytes);
			let uniforms = uniform! {
				active_location: active_location,
				matrix: mtx,
				scale: self.active_scale_ratio,
				self_location: self_location,
				tex: img,
				td: tst_distance,
			};
			target.draw(&buffer.buffer, &self.indices, &program, &uniforms,
						&Default::default()).unwrap();
		}

		let screen_dims = self.display.get_framebuffer_dimensions();

		for row in &self.row_titles {
			stdout().flush().unwrap();
			self.text_renderer.queue(Section{
				text: &row.title.to_string(),
				bounds: (screen_dims.0 as f32, screen_dims.1 as f32 / 2.0),
				color: [1.0, 1.0, 1.0, 1.0],
				scale: Scale::uniform(24.0),
				screen_position: (row.pos[0] * self.width as f32, (row.pos[1] - (vertical * 0.5)) * self.height as f32),
				..Section::default()
			});
		}



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
						_ => ()
					},
					_ => (),
				}
				_ => ()
			}
		}
	}
}

#[derive(Debug)]
pub struct VertexBufferContainer {
	pub buffer: VertexBuffer<VertexData>,
	pub self_location: [f32; 2],
	pub texture_bytes: bytes::Bytes,
	pub texture_id: String,
	pub tst_distance: [f32; 2],
}

pub struct ActiveLocation {
	debounce: Duration,
	last_tick: Instant,
	x: i32,
	y: i32,
	x_limit: i32,
	y_limit: i32,
	virtual_x: i32,
	virtual_y: i32,
	virtual_x_limit: i32,
	virtual_y_limit: i32,
	x_cache: HashMap<String, Vec<i32>>,
}

impl ActiveLocation {
	pub fn to_vec(&self) -> [f32; 2] { [self.x as f32, self.y as f32] }
	pub fn set_max_tiles(&mut self, limit: i32) {
		self.x_limit = limit;
	}
	pub fn set_max_rows(&mut self, limit: i32) {
		self.y_limit = limit;
	}
	pub fn move_up(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			let row_snapshot: Vec<i32> = ((self.x - self.virtual_x)..(self.x - self.virtual_x) + 5).collect();
			self.x_cache.insert(self.y.to_string(), row_snapshot.clone());
			self.virtual_y = max(0, self.virtual_y - 1);
			self.y = max(0, self.y - 1);

			let target_row_snapshot = self.x_cache.entry(self.y.to_string()).or_insert(row_snapshot).to_owned();
			self.x = target_row_snapshot[self.virtual_x as usize];
			self.last_tick = Instant::now();
		}
	}
	pub fn move_down(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			let row_snapshot: Vec<i32> = ((self.x - self.virtual_x)..(self.x - self.virtual_x) + 5).collect();
			self.x_cache.insert(self.y.to_string(), row_snapshot.clone());
			self.virtual_y = min(self.virtual_y_limit - 1, self.virtual_y + 1);
			self.y = min(self.y_limit - 1, self.y + 1);

			let target_row_snapshot = self.x_cache.entry(self.y.to_string()).or_insert((0..self.virtual_x_limit).collect()).to_owned();
			self.x = target_row_snapshot[self.virtual_x as usize];
			self.last_tick = Instant::now();
		}
	}
	pub fn move_left(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.virtual_x = max(0, self.virtual_x - 1);
			self.x = max(0, self.x - 1);
			self.last_tick = Instant::now();
		}
	}
	pub fn move_right(&mut self) {
		if self.last_tick.elapsed() >= self.debounce {
			self.virtual_x = min(self.virtual_x_limit - 1, self.virtual_x + 1);
			self.x = min(self.x_limit - 1, self.x + 1);
			self.last_tick = Instant::now();
		}
	}
}

pub struct ScreenRowTitle {
	title: String,
	pos: [f32; 2]
}





