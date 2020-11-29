mod rendering;
mod types;
mod data;

#[macro_use]
extern crate glium;
extern crate image;

use crate::data::{prepare_data};
use crate::rendering::screen::{Screen};
use crate::rendering::shapes::{Vertex, Row};
use crate::types::DSSData;

fn main() {
	let data = prepare_data().unwrap();
	render(data);
}

fn render(data: DSSData) {
	let event_loop = glium::glutin::event_loop::EventLoop::new();
	let mut renderer = Screen::new(1440, 900, &event_loop);

	renderer.use_default_shaders();
	let mut active_rows = vec![];

	for container in data.data.StandardCollection.containers.iter() {
		if let Some(_) = &container.set.items {
			active_rows.push(container);
		}
	}

	renderer.set_active_rows_count(active_rows.len() as f32);

	for (i, &container) in active_rows.iter().enumerate() {
		let mut row = Row::new(i, active_rows.len());
		row.add_placeholder_tiles(container.set.items.as_ref().unwrap().iter().len() as i32);
		renderer.add_shapes(row.tiles.unwrap());
	}

	print!("{}", "\n".repeat(active_rows.len()));
	event_loop.run(move |ev, _, control_flow| {
		renderer.render(&ev, control_flow);
	});
}
