use crate::round;

pub struct Game {
    pub controller: round::Controller,
}

impl Game {
    pub fn welcome(&self) {
        println!("1 2 3 ");
        println!("4 5 6 ");
        println!("7 8 9 ");
    }

    pub fn playing(&self) -> bool {
        self.controller.has_next_round()
    }

    pub fn play(&mut self) {
        self.controller.run();
    }

    pub fn gameover(&self) {
        println!("---------");
        println!("Game Over");
    }
}
