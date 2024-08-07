use macroquad::prelude::next_frame;

use dyhra::game::prelude::Game;

#[macroquad::main("Dyhra")]
async fn main() {
    let game = Game::new().await;

    loop {
        game.events();
        game.update();
        game.draw();

        next_frame().await;
    }
}
