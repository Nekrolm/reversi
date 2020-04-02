mod game;
mod view;


fn main() {


    use view::stdio::StdIOPlayer;
    use view::stdio::PlayerId;
    use view::curses::{CursesPlayer, NCursesWindow};
    use std::rc::Rc;

    let win = Rc::new(NCursesWindow::new());
    let mut game = game::Game::new(
        Box::new(CursesPlayer::new(PlayerId::Black, win.clone())),
        Box::new(CursesPlayer::new(PlayerId::White, win)));

    game.run();
}
