
use glium::glutin;
use glium::Surface;

pub struct MainLoop {
	events_loop: &mut glutin::EventsLoop,
	window: &glutin::Window,
	
}

impl MainLoop {

	fn new() -> Self {
		return MainLoop {
			events_loop: glutin::EventsLoop::new()
		}	
	}

}
