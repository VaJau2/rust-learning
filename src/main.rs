use crate::game::Game;

mod game;
mod game_objects;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let mut game = Game::init();

    while game.is_running() {
        game.update();
    }
}
