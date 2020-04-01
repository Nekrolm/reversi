mod game;
mod view;



fn main() {
    use view::stdio::StdIOPlayer;
    use view::stdio::PlayerId;

    let r = &(0..5);
    for x in r.clone() {
        for y in r.clone() {
            println!();
        }
    }

    let mut game = game::Game::new(
        Box::new(StdIOPlayer::new(PlayerId::Black)),
        Box::new(StdIOPlayer::new(PlayerId::White)));

    game.run();
}
