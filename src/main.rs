extern crate glium;
extern crate image;

mod app;
mod gfx;
mod assets;

use app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
