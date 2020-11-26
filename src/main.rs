mod rendering;

#[macro_use]
extern crate glium;
extern crate image;

pub use crate::rendering::screen::*;
pub use crate::rendering::shapes::{get_vertex_buffer, Vertex};



fn main() {
    let mut renderer = Screen::new(1440, 900);

    let v1 = get_vertex_buffer(0.50, 0.50);
    let v2 = get_vertex_buffer(0.50, -0.50);

    renderer.add_shape(&v1);
    renderer.add_shape(&v2);
    renderer.run();
}
