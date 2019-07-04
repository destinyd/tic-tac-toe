use crate::game;

pub struct Engine {
    pub game: game::Game,
}

impl Engine {
    pub fn init(&mut self) {
    }

    pub fn start(&mut self) {
        self.game.welcome();
        loop {
            self.game.play();
            if !self.game.playing() {
                break
            }
        }
        self.game.gameover();
    }

    pub fn recycle(&self) {
    }
}
