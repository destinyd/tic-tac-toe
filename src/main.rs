mod round;
mod game;
mod engine;

fn main() {
    let mut obj_engine = engine::Engine {
        game: game::Game {
            controller : round::Controller {
                input_num: 0, round: 1, user: 1, points: [0; 9]
            },
        }
    };

    obj_engine.init();
    obj_engine.start();
    obj_engine.recycle();
}
