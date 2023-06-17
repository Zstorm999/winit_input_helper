use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, _| {
        if input.update(&event) {
            if input.mouse_held(0) {
                println!("mouse at {:?}", input.mouse())
            }
        }
    });
}
