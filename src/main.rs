mod rendering;
mod data;

#[macro_use]
extern crate glium;
extern crate glium_glyph;
extern crate image;

use crate::data::{prepare_data};
use crate::rendering::screen::{Screen};
use crate::rendering::shapes::{Vertex, Row};
use data::types::{DSSData};
use std::collections::HashMap;

fn main() {
	let data = prepare_data().unwrap();
	render(data);
}

fn render(data: DSSData) {
	let event_loop = glium::glutin::event_loop::EventLoop::new();
	let mut renderer = Screen::new(1600, 900, &event_loop);
	let mut texture_cache: HashMap<String, glium::texture::SrgbTexture2d> = HashMap::new();

	renderer.use_default_tile_shaders();
	let mut active_rows = vec![];

	for container in data.data.StandardCollection.containers.into_iter() {
		if container.set.items.is_some() {
			active_rows.push(container);
		}
	}

	renderer.set_active_rows(active_rows);

	for (i, container) in renderer.rows.clone().iter().enumerate() {
		let mut row = Row::new(i, renderer.rows_count as usize);
		row.add_tiles(&container);
		row.add_row_title(&container);
		renderer.add_row(row);
	}

	event_loop.run(move |ev, _, control_flow| {
		renderer.render(&ev, control_flow, &mut texture_cache);
	});
}
