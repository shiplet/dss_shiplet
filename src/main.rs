mod rendering;
mod types;
mod data;

#[macro_use]
extern crate glium;
extern crate glium_glyph;
extern crate image;

use crate::data::{prepare_data};
use crate::rendering::screen::{Screen};
use crate::rendering::shapes::{Vertex, Row};
use crate::types::{DSSData};

const DEBUG: bool = true;

fn main() {
	let data = prepare_data().unwrap();
	render(data);
}

fn render(data: DSSData) {
	let event_loop = glium::glutin::event_loop::EventLoop::new();
	let mut renderer = Screen::new(1440, 900, &event_loop);

	renderer.use_default_shaders();
	let mut active_rows = vec![];

	for container in data.data.StandardCollection.containers.into_iter() {
		if container.set.items.is_some() {
			active_rows.push(container);
		}
	}

	renderer.set_active_rows(active_rows);

	for (i, container) in renderer.active_rows.clone().iter().enumerate() {
		let mut row = Row::new(i, renderer.active_rows_count as usize);
		row.add_tiles(&container);
		row.add_row_title(&container);
		renderer.add_row(row);
	}

	if DEBUG {
		print!("{}", "\n".repeat(renderer.active_rows_count as usize)); // this is for the ANSI escape codes used later on
	}
	event_loop.run(move |ev, _, control_flow| {
		renderer.render(&ev, control_flow);
	});
}
