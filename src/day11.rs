use std::{fs, u8};
use std::collections::BTreeMap;

pub fn day11() {
    let string = fs::read_to_string("dumbo.txt").unwrap();
    let lines = string.lines();

    let mut state = [[0; 10]; 10];

    for (x, line) in lines.enumerate() {
        for (y, c) in line.chars().enumerate() {
            state[x][y] = c.to_string().parse::<i32>().unwrap();
        }
    }

    let mut global_flashes = 0;

    let mut part1 = state.clone();
    for _ in 0..100 {
        global_flashes += one_step(&mut part1);
    }

    let mut steps_taken = 0;
    let mut part2 = state.clone();
    loop {
        steps_taken += 1;
        let flashes = one_step(&mut part2);
        if flashes == 100 {break}
    }

    println!("\nflashes {}", global_flashes);
    println!("\nsteps until 100 flashes {}", steps_taken);
}

fn one_step(input: &mut [[i32; 10]; 10]) -> i32 {
    let mut flashes_in_step = 0;
    for x in 0..10 {
        for y in 0..10 {
            input[x][y] += 1;
        }
    }

    loop {
        let mut flashes = 0;
        for x in 0..10 {
            for y in 0..10 {
                if &&input[x][y] > &&9 {
                    if x != 0 && y != 9 {
                        input[x - 1][y + 1] += 1;
                    }
                    if y != 9 {
                        input[x][y + 1] += 1;
                    }
                    if x != 9 && y != 9 {
                        input[x + 1][y + 1] += 1;
                    }

                    if x != 0 {
                        input[x - 1][y] += 1;
                    }
                    if x != 9 {
                        input[x + 1][y] += 1;
                    }
                    if x != 0 && y != 0 {
                        input[x - 1][y - 1] += 1;
                    }
                    if y != 0 {
                        input[x][y - 1] += 1;
                    }
                    if x != 9 && y != 0 {
                        input[x + 1][y - 1] += 1;
                    }
                    flashes += 1;
                    flashes_in_step += 1;
                    input[x][y] = i32::MIN;
                }
            }
        }
        if flashes == 0 { break; }
    }

    for x in 0..10 {
        for y in 0..10 {
            if input[x][y] < 0 { input[x][y] = 0; }
        }
    }

    flashes_in_step
}
