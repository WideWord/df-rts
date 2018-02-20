extern crate glium;

mod app;
mod gfx;

use app::MainLoop;

fn main() {
	let mut main_loop = MainLoop::new();
	main_loop.run();
}
