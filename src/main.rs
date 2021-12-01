use std::fs;

fn main() {
    println!("advent of code day 1 -p1");

    let string = fs::read_to_string("depths.txt").unwrap();
    let mut lines = string.lines();

    let mut increased = 0;
    let mut decreased = 0;
    let mut previous_depth = lines.next().unwrap().parse::<i32>().unwrap();

    for line in lines {
        let num = line.parse::<i32>().unwrap();
        if num > previous_depth {
            increased += 1;
        }
        if num < previous_depth {
            decreased += 1;
        }
        previous_depth = num
    }

    println!("amount of times increased: {}", increased);
    println!("amount of times decreased: {}", decreased);
    println!("----------------------");
    println!("advent of code day 1 -p2");

    let string = fs::read_to_string("depths.txt").unwrap();
    let mut lines = string.lines();

    let mut sliding_windows = Vec::new();

    let mut elem0 = 0;
    let mut elem1 = 0;
    let mut elem2 = 0;

    for line in lines {
        let num = line.parse::<i32>().unwrap();
        elem2 = elem1;
        elem1 = elem0;
        elem0 = num;

        sliding_windows.push(SlidingWindow(elem0, elem1, elem2));
    }

    let mut previous_window_sum = sliding_windows.get(0).unwrap().sum();
    increased = 0;
    decreased = 0;

    sliding_windows.iter().skip(3).for_each(| item | {
        if item.sum() > previous_window_sum {
            increased += 1;
        }
        if item.sum() < previous_window_sum {
            decreased += 1;
        }
        previous_window_sum = item.sum();
        println!("{:#?}", item);
    });

    println!("amount of times increased: {}", increased);
    println!("amount of times decreased: {}", decreased);
}

#[derive(Debug)]
struct SlidingWindow(i32, i32, i32);

impl SlidingWindow {
    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2
    }
}
