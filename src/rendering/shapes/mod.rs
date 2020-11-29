#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub data: VertexData,
	pub self_location: Option<[f32; 2]>,
	pub translate_dist: Option<[f32; 2]>,
}

#[derive(Copy, Clone, Debug)]
pub struct VertexData {
	pub position: [f32; 2],
	pub tex_coords: [f32; 2],
}

implement_vertex!(VertexData, position, tex_coords);

#[derive(Debug, Clone)]
pub struct Row {
	pub margin_top: f32,
	pub margin_left: f32,
	pub index: usize,
	pub total: usize,
	pub tiles: Option<Vec<Vec<Vertex>>>,
}

impl Row {
	pub fn new(index: usize, total: usize) -> Row {
		Row {
			index,
			total,
			margin_top: 0.35,
			margin_left: 0.25,
			tiles: None,
		}
	}
	pub fn add_placeholder_tiles(&mut self, length: i32) {
		if let Some(_t) = &self.tiles {
			println!("tiles already added")
		} else {
			let mut tiles = vec![];
			for n in (length * -1)..0 {
				let og_index = length - n.abs();
				let x_const = (og_index) as f32 * 0.30;
				let x_box_comp = 1.0 - (self.margin_left + 0.0325);
				let padding_left = x_const * self.margin_left;

				let y_const = self.index as f32 * 0.30;
				let y_box_comp = 1.0 - (1.0 - (self.margin_left + 0.15));
				let padding_top = self.index as f32 * self.margin_top;

				let x = (x_const + padding_left) - x_box_comp;
				let y = 1.0 - (y_const + padding_top) - y_box_comp;

				let tile = create_tile(x, y, (length - n.abs()) as f32, self.index as f32);
				tiles.push(tile);
			}
			self.tiles = Some(tiles);
		}
	}
}

pub fn create_tile(x: f32, y: f32, col: f32, row: f32) -> Vec<Vertex> {
	let x_trans = x * -1.0;
	let y_trans = y * -1.0;
	let vertex1 = Vertex { self_location: Some([col, row]), translate_dist: Some([x_trans, y_trans]), data: VertexData { position: [-0.15 + x,  0.15 + y],  tex_coords: [ 0.0, 0.99] }};
	let vertex2 = Vertex { self_location: None, translate_dist: None, data: VertexData { position: [-0.15 + x, -0.15 + y],  tex_coords: [ 0.0,  0.0] }};
	let vertex3 = Vertex { self_location: None, translate_dist: None, data: VertexData { position: [ 0.15 + x, -0.15 + y],  tex_coords: [0.99,  0.0] }};

	let vertex4 = Vertex { self_location: None, translate_dist: None, data: VertexData { position: [-0.15 + x,  0.15 + y], tex_coords: [ 0.0, 0.99] }};
	let vertex5 = Vertex { self_location: None, translate_dist: None, data: VertexData { position: [ 0.15 + x,  0.15 + y], tex_coords: [0.99, 0.99] }};
	let vertex6 = Vertex { self_location: None, translate_dist: None, data: VertexData { position: [ 0.15 + x, -0.15 + y], tex_coords: [0.99,  0.0] }};
	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
	shape
}