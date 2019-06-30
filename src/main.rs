use std::io;

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

fn main() {
    println!("1 2 3 ");
    println!("4 5 6 ");
    println!("7 8 9 ");

    let mut step = 0;
    let mut user = 1;
    // FIXME use array instead of vec
    let mut points = [0; 9];

    loop {
        // 1 get point
        let num: i32 = step_input(user);

        // 2 put point
        if !step_put(&mut points, num, user) { continue; }

        // 3 output points
        output(points);

        // 4 Judge the winning or losing
        if is_game_over(&mut points, user, step, num) {
            println!("Game over");
            break;
        }

        // 5 initial next
        step_next(&mut user, &mut step);
    }
}

fn output(points: [i32; 9]) {
    let mut i = 0;
    for data in points.iter() {
        print!("{} ", data);
        if i % 3 == 2 {
            print!("\r\n");
        }
        i = i + 1;
    }
}

fn is_win(points: &mut [i32; 9]) -> bool {
    // 竖排
    if points[0] == points[3] && points[0] == points[6] && points[0] > 0 {
        return true;
    }
    if points[1] == points[4] && points[1] == points[7] && points[1] > 0 {
        return true;
    }
    if points[2] == points[5] && points[2] == points[8] && points[2] > 0 {
        return true;
    }

    // 横排
    if points[0] == points[1] && points[0] == points[2] && points[0] > 0 {
        return true;
    }
    if points[3] == points[4] && points[3] == points[5] && points[3] > 0 {
        return true;
    }
    if points[6] == points[7] && points[6] == points[8] && points[6] > 0 {
        return true;
    }

    // 斜
    if points[4] > 0 {
        if points[0] == points[4] && points[0] == points[8] {
            return true;
        }
        if points[2] == points[4] && points[2] == points[6] {
            return true;
        }
    }

    return false;
}

fn step_input(user: i32) -> i32 {
    println!("请输入 user{} 放置位置，或输入 0 结束", user);
    return numin!();
}

fn step_put(points: &mut [i32; 9] ,num: i32, user: i32) -> bool {
    let index = num as usize - 1;
    let point = points[index];

    if point != 0 {
        println!("位置 {} 已放置，请重新输入!", num);
        return false;
    }
    points[index] = user;
    return true;
}

fn is_game_over(points: &mut [i32; 9], user: i32, step: i32, num: i32) -> bool {
    if step >= 5 - 1 {
        if is_win(points) {
            println!("user{} win!!!", user);
            return true;
        }
    }

    if step == 8 || num == 0 {
        return true
    }

    return false;
}

fn step_next(user: &mut i32, step: &mut i32) {
    if *user == 1 {
        *user = 2;
    } else {
        *user = 1;
    }
    *step = *step + 1;
}
