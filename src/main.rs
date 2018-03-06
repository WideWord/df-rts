#[macro_use] extern crate glium;
extern crate image;
extern crate cgmath;
extern crate assimp;
#[macro_use] extern crate enum_map;
#[macro_use] extern crate enum_map_derive;

mod app;
mod assets;
mod gfx;
mod math;
mod terrain;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
