#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct Row {
    pub padding: [f32; 4],
    pub index: i32,
    pub total: i32,
    pub tiles: Option<Vec<Vec<Vertex>>>,
}

impl Row {
    pub fn new(index: i32, total: i32) -> Row {
       Row {
           index,
           total,
           padding: [0.35, 0.15, 0.15, 0.15],
           tiles: None,
       }
    }
    pub fn add_dummy_tiles(&mut self, length: i32) {
        if let Some(_t) = &self.tiles {
            println!("tiles already added")
        } else {
            let mut tiles = vec![];
            for n in (length * -1)..0 {
                let x_const = (length - n.abs()) as f32 * 0.30;
                let y_const = self.index as f32 * 0.30;
                let x_box_comp = 1.0 - (self.padding[3] + 0.15);
                let y_box_comp = 1.0 - (1.0 - self.padding[0]);
                let padding_left = x_const * self.padding[3];
                let padding_top = y_const * self.padding[0];

                let x = (x_const + padding_left) - x_box_comp;
                let y = 1.0 - (y_const + padding_top) - y_box_comp;

                let tile = create_tile(x, y);
                tiles.push(tile);
            }
            self.tiles = Some(tiles);
        }
    }
}

implement_vertex!(Vertex, position, tex_coords);

// pub fn create_row(length: u32) -> Row {}

pub fn create_tile(x: f32, y: f32) -> Vec<Vertex> {
    let vertex1 = Vertex { position: [-0.15 + x,  0.15 + y], tex_coords: [0.0, 0.99]};
    let vertex2 = Vertex { position: [-0.15 + x, -0.15 + y], tex_coords: [0.0, 0.0]};
    let vertex3 = Vertex { position: [ 0.15 + x, -0.15 + y], tex_coords: [0.99, 0.0]};

    let vertex4 = Vertex { position: [-0.15 + x,  0.15 + y], tex_coords: [0.0, 0.99]};
    let vertex5 = Vertex { position: [ 0.15 + x,  0.15 + y], tex_coords: [0.99, 0.99]};
    let vertex6 = Vertex { position: [ 0.15 + x, -0.15 + y], tex_coords: [0.99, 0.0]};
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    shape
}