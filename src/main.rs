extern crate glium;

mod app;

fn main() {
	let mut main_loop = app::MainLoop::new();
	main_loop.run();
}
