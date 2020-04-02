mod game;
mod view;

use winit:: {
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};



fn main() {

    // let event_loop = EventLoop::new();
    //
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    //
    // event_loop.run(move |event, _, control_flow| {
    //     *control_flow = ControlFlow::Wait;
    //
    //     match event {
    //         Event::WindowEvent {
    //             event: WindowEvent::CloseRequested,
    //             window_id,
    //         } if window_id == window.id() => *control_flow = ControlFlow::Exit,
    //         _ => (),
    //     }
    // });

    use view::stdio::StdIOPlayer;
    use view::stdio::PlayerId;

    let mut game = game::Game::new(
        Box::new(StdIOPlayer::new(PlayerId::Black)),
        Box::new(StdIOPlayer::new(PlayerId::White)));

    futures::executor::block_on(game.run());
}
