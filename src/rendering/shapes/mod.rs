#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn get_vertex_buffer(x: f32, y: f32) -> Vec<Vertex> {
    let vertex1 = Vertex { position: [-0.15 - x,  0.15 + y], tex_coords: [0.0, 0.99]};
    let vertex2 = Vertex { position: [-0.15 - x, -0.15 + y], tex_coords: [0.0, 0.0]};
    let vertex3 = Vertex { position: [ 0.15 - x, -0.15 + y], tex_coords: [0.99, 0.0]};

    let vertex4 = Vertex { position: [-0.15 - x,  0.15 + y], tex_coords: [0.0, 0.99]};
    let vertex5 = Vertex { position: [ 0.15 - x,  0.15 + y], tex_coords: [0.99, 0.99]};
    let vertex6 = Vertex { position: [ 0.15 - x, -0.15 + y], tex_coords: [0.99, 0.0]};
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    shape
}