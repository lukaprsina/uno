use uno::Game;

#[test]
fn play_game() {
    for _ in 0..10_000 {
        let mut game = Game::new(12);

        game.start(500);
    }
}
