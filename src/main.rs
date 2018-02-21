extern crate glium;
extern crate image;
extern crate cgmath;

mod app;
mod assets;
mod gfx;
mod math;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
