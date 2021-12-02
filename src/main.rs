use std::fmt::{Debug, Display, Formatter};
use std::fs;

fn main() {
    // println!("advent of code day 1 -p1");
    //
    // let string = fs::read_to_string("depths.txt").unwrap();
    // let mut lines = string.lines();
    //
    // let mut increased = 0;
    // let mut decreased = 0;
    // let mut previous_depth = lines.next().unwrap().parse::<i32>().unwrap();
    //
    // for line in lines {
    //     let num = line.parse::<i32>().unwrap();
    //     if num > previous_depth {
    //         increased += 1;
    //     }
    //     if num < previous_depth {
    //         decreased += 1;
    //     }
    //     previous_depth = num
    // }
    //
    // println!("amount of times increased: {}", increased);
    // println!("amount of times decreased: {}", decreased);
    // println!("----------------------");
    // println!("advent of code day 1 -p2");
    //
    // let string = fs::read_to_string("depths.txt").unwrap();
    // let mut lines = string.lines();
    //
    // let mut sliding_windows = Vec::new();
    //
    // let mut elem0 = 0;
    // let mut elem1 = 0;
    // let mut elem2 = 0;
    //
    // for line in lines {
    //     let num = line.parse::<i32>().unwrap();
    //     elem2 = elem1;
    //     elem1 = elem0;
    //     elem0 = num;
    //
    //     sliding_windows.push(SlidingWindow(elem0, elem1, elem2));
    // }
    //
    // let mut previous_window_sum = sliding_windows.get(0).unwrap().sum();
    // increased = 0;
    // decreased = 0;
    //
    // sliding_windows.iter().skip(3).for_each(| item | {
    //     if item.sum() > previous_window_sum {
    //         increased += 1;
    //     }
    //     if item.sum() < previous_window_sum {
    //         decreased += 1;
    //     }
    //     previous_window_sum = item.sum();
    //     println!("{:#?}", item);
    // });
    //
    // println!("amount of times increased: {}", increased);
    // println!("amount of times decreased: {}", decreased);



    let string = fs::read_to_string("dive.txt").unwrap();
    let mut lines = string.lines();

    let commands: Vec<Pilot> = lines.map(|line| {
        Pilot::from(line)
    }).collect();

    let mut pos = Position::new();
    pos.apply(commands);

    println!("{:#?}", pos);
    println!("{:#?}", pos.aoc_solution());
}

#[derive(Debug)]
enum Pilot {
    Forward(i32),
    Down(i32),
    Up(i32),
    Nop,
}

impl Pilot {
    fn from(str: &str) -> Pilot {
        let parts: Vec<&str> = str.split(' ').collect();
        let param = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => Pilot::Forward(param),
            "down" => Pilot::Down(param),
            "up" => Pilot::Up(param),
            &_ => Pilot::Nop
        }
    }
}


#[derive(Debug)]
struct Position {
    horizontal_position: i32,
    depth: i32,
}

impl Position {
    fn new() -> Position {
        Position { horizontal_position: 0, depth: 0 }
    }

    fn apply(&mut self, cmds: impl IntoIterator<Item=Pilot>) {
        for cmd in cmds {
            match cmd {
                Pilot::Forward(v) => {self.horizontal_position += v}
                Pilot::Down(v) => {self.depth += v}
                Pilot::Up(v) => {self.depth -=v}
                _ => ()
            }
        }
    }

    fn aoc_solution(&self) -> i32 {
        self.depth * self.horizontal_position
    }
}

#[derive(Debug)]
struct SlidingWindow(i32, i32, i32);

impl SlidingWindow {
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}
