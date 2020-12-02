use reqwest::{Error as rqErr};
use crate::types::{Container, Item};
use std::io;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Vertex {
	pub data: VertexData,
	pub self_location: Option<[f32; 2]>,
	pub texture: Option<TextureData>,
	pub tst_distance: Option<[f32; 2]>,
}

#[derive(Clone, Debug)]
pub struct TextureData {
	pub texture_bytes: bytes::Bytes,
	pub texture_id: String
}

#[derive(Copy, Clone, Debug)]
pub struct VertexData {
	pub position: [f32; 2],
	pub tex_coords: [f32; 2],
}

implement_vertex!(VertexData, position, tex_coords);

#[derive(Debug, Clone)]
pub struct Row {
	pub index: usize,
	pub margin_left: f32,
	pub margin_top: f32,
	pub tiles: Option<Vec<Vec<Vertex>>>,
	pub title: String,
	pub title_pos: [f32; 2],
	pub total: usize,
}

impl Row {
	pub fn new(index: usize, total: usize) -> Row {
		Row {
			index,
			margin_left: 0.25,
			margin_top: 0.25,
			tiles: None,
			title: String::new(),
			title_pos: [0.0; 2],
			total,
		}
	}

	pub fn add_tiles(&mut self, container: &Container) {
		let items = container.set.items.as_ref().unwrap();
		let length = items.len() as i32;
		let mut tiles = vec![];
		for n in -length..0 {
			print!("\rfetching images: [{}>{}]", "=".repeat((length + n) as usize), " ".repeat((-n - 1) as usize));
			io::stdout().flush().unwrap();
			let raw_img = self.get_image(&container.set.items.as_ref().unwrap()[-(n+1) as usize]).unwrap();
			let og_index = length - n.abs();
			let x = self.get_x_pos(og_index);
			let y = self.get_y_pos();
			let tile = create_tile(x, y, (length - n.abs()) as f32, self.index as f32, raw_img);
			tiles.push(tile);
		}
		print!("\n");
		self.tiles = Some(tiles);
	}

	pub fn add_row_title(&mut self, container: &Container) {
		if let Some(title_top) = &container.set.text.title.full {
			if let Some(title) = &title_top.set {
				self.title = title.default.content.to_owned();
				let x_pos = self.margin_left * 0.25;
				let y_multiplier = (self.index as f32 + 0.25) + (self.index as f32 * 0.10);
				let y_pos = 0.25 * y_multiplier;
				self.title_pos = [x_pos, y_pos];
			}
		}
	}

	fn get_image(&self, item: &Item) -> Result<TextureData, rqErr> {
		let url = &item.image.tile["1.78"];
		let mut img_url = String::new();
		if let Some(s) = &url.series {
			img_url = s.default.url.to_string();
		}
		if let Some(p) = &url.program {
			img_url = p.default.url.to_string();
		}
		if let Some(d) = &url.default {
			img_url = d.default.url.to_string();
		}
		let img_bytes = reqwest::blocking::get(&img_url)?.bytes()?;
		Ok(TextureData{
			texture_bytes: img_bytes,
			texture_id: img_url
		})
	}

	fn get_x_pos(&self, og_index: i32) -> f32 {
		let x_const = (og_index) as f32 * 0.30;
		let x_box_comp = 1.0 - (self.margin_left + 0.0325);
		let padding_left = x_const * self.margin_left;
		(x_const + padding_left) - x_box_comp
	}

	fn get_y_pos(&self) -> f32 {
		let y_const = self.index as f32 * 0.30;
		let y_box_comp = 1.0 - (1.0 - (self.margin_top + 0.15));
		let padding_top = self.index as f32 * self.margin_top;
		1.0 - (y_const + padding_top) - y_box_comp
	}
}

pub fn create_tile(x: f32, y: f32, col: f32, row: f32, tex: TextureData) -> Vec<Vertex> {
	let x_trans = x * -1.0;
	let y_trans = y * -1.0;
	let vertex1 = Vertex { self_location: Some([col, row]), texture: Some(tex), tst_distance: Some([x_trans, y_trans]), data: VertexData { position: [-0.15 + x,  0.15 + y],  tex_coords: [ 0.0, 0.99] }};
	let vertex2 = Vertex { self_location: None, texture: None, tst_distance: None, data: VertexData { position: [-0.15 + x, -0.15 + y],  tex_coords: [ 0.0,  0.0] }};
	let vertex3 = Vertex { self_location: None, texture: None, tst_distance: None, data: VertexData { position: [ 0.15 + x, -0.15 + y],  tex_coords: [0.99,  0.0] }};

	let vertex4 = Vertex { self_location: None, texture: None, tst_distance: None, data: VertexData { position: [-0.15 + x,  0.15 + y], tex_coords: [ 0.0, 0.99] }};
	let vertex5 = Vertex { self_location: None, texture: None, tst_distance: None, data: VertexData { position: [ 0.15 + x,  0.15 + y], tex_coords: [0.99, 0.99] }};
	let vertex6 = Vertex { self_location: None, texture: None, tst_distance: None, data: VertexData { position: [ 0.15 + x, -0.15 + y], tex_coords: [0.99,  0.0] }};
	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
	shape
}