mod game;
mod view;
mod player;



fn main() {

    use view::stdio::StdIOView;
    use player::user_player::UserPlayer;
    use player::dummy_player::DummyPlayer;
    use player::PlayerId;
    use view::curses::{NCursesView, NCursesWindow};
    use std::rc::Rc;


    let win = Rc::new(NCursesWindow::new());
    let view = Rc::new(NCursesView::new(win));


    let mut game = game::Game::new(
        Box::new(UserPlayer::new(PlayerId::Black, view.clone())),
        Box::new(DummyPlayer::new(PlayerId::White)));

    game.run();
}
