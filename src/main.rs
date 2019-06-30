macro_rules! numin {
    () =>{
        {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().parse().unwrap()
        }
    };
}

struct GameStruct {
    input_num: usize,
    step: i32,
    user: i32,
    points: [i32; 9],
}

trait Game {
    fn init(&self);
    fn playing(&self) -> bool;
    fn play(&mut self);

    fn input(&mut self);
    fn put_piece(&mut self) -> bool;
    fn output(&self);
    fn is_win(&self) -> bool;
    fn next_step(&mut self);
}

impl Game for GameStruct {
    fn init(&self) {
        println!("1 2 3 ");
        println!("4 5 6 ");
        println!("7 8 9 ");
    }

    fn playing(&self) -> bool {
        if self.step > 1 && self.input_num == 0 {
            return false;
        }

        if self.step <= 9 && !self.is_win() {
            return true
        }

        return false;
    }

    fn play(&mut self) {
        println!("------step: {}--------", self.step);

        self.input();

        if self.put_piece() {
            self.output();

            if self.is_win() {
                println!("user{} win!!!", self.user);
            } else {
                self.next_step();
            }
        }
    }

    fn input(&mut self) {
        println!("请输入 user{} 放置位置，或输入 0 结束", self.user);
        self.input_num = numin!();
    }

    fn put_piece(&mut self) -> bool {
        let index = self.input_num - 1;
        let point = self.points[index];

        if point != 0 {
            println!("位置 {} 已放置，请重新输入!", self.input_num);
            return false;
        }
        self.points[index] = self.user;
        return true;
    }

    fn output(&self) {
        let mut i = 0;
        for data in self.points.iter() {
            match data {
                1 => print!("|O"),
                2 => print!("|X"),
                _ => print!("| "),
            };
            if i % 3 == 2 {
                print!("|\r\n");
            }
            i = i + 1;
        }
    }

    fn is_win(&self) -> bool {
        if self.step < 5 {
            return false;
        }

        // 竖排
        if self.points[0] == self.points[3] && self.points[0] == self.points[6] && self.points[0] > 0 {
            return true;
        }
        if self.points[1] == self.points[4] && self.points[1] == self.points[7] && self.points[1] > 0 {
            return true;
        }
        if self.points[2] == self.points[5] && self.points[2] == self.points[8] && self.points[2] > 0 {
            return true;
        }

        // 横排
        if self.points[0] == self.points[1] && self.points[0] == self.points[2] && self.points[0] > 0 {
            return true;
        }
        if self.points[3] == self.points[4] && self.points[3] == self.points[5] && self.points[3] > 0 {
            return true;
        }
        if self.points[6] == self.points[7] && self.points[6] == self.points[8] && self.points[6] > 0 {
            return true;
        }

        // 斜
        if self.points[4] > 0 {
            if self.points[0] == self.points[4] && self.points[0] == self.points[8] {
                return true;
            }
            if self.points[2] == self.points[4] && self.points[2] == self.points[6] {
                return true;
            }
        }

        return false;
    }

    fn next_step(&mut self) {
        if self.user == 1 {
            self.user = 2;
        } else {
            self.user = 1;
        }
        self.step = self.step + 1;
    }
}

struct GameBuilder {
    input_num: usize,
    step: i32,
    user: i32,
    points: [i32; 9],
}

impl GameBuilder {
    fn new() -> GameBuilder {
        GameBuilder { input_num: 0, step: 1, user: 1, points: [0; 9] }
    }

    fn finalize(&self) -> GameStruct {
        return GameStruct {
            input_num: self.input_num,
            step: self.step,
            user: self.user,
            points: self.points,
        }
    }
}

fn main() {
    let mut game = GameBuilder::new().finalize();

    game.init();

    while game.playing() {
        game.play();
    }

    println!("---------");
    println!("Game Over");
}
