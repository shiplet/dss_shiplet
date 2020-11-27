mod rendering;

#[macro_use]
extern crate glium;
extern crate image;

pub use crate::rendering::screen::*;
pub use crate::rendering::shapes::{create_tile, Vertex, Row};


fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let mut renderer = Screen::new(1440, 900, &event_loop);

    renderer.use_default_shaders();

    for n in 0..5 {
        let mut row = Row::new(n, 5);
        row.add_dummy_tiles(10);
        renderer.add_shapes(row.tiles.unwrap());
    }

    event_loop.run(move |ev, _, control_flow| {
        renderer.render(&ev, control_flow);
    });
}
