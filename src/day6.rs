use std::fmt::{Display, Formatter};
use std::fs;

pub fn day6() {
    let string = fs::read_to_string("fishies.txt").unwrap();
    let mut fishies: Vec<usize> = string.split(',').map(|i| i.parse().unwrap()).collect();
    let mut o_fishies: Vec<usize> = vec![3, 4, 3, 1, 2];

    let days = 257;

    let mut fish: [i32; 9] = Default::default();
    for age in 0..fishies.len() {
        fish[age] += 1;
    }

    for i in 0..days {
        println!("day {}, amount of fish: {}", i, &fishies.len());
        fish[(i + 7) % 9] += fish[i % 9];
    }
}

pub fn day6_old() {
    let string = fs::read_to_string("fishies.txt").unwrap();
    let mut fishies: Vec<Lanternfish> = string.split(',').map(Lanternfish::from).collect();

    let days = 256;


    for i in 0..days + 1 {
        println!("day {}, {} fish", i, fishies.len());
        for i in 0..fishies.len() {
            fishies[i].pass_day().map(|f| fishies.push(f));
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Lanternfish {
    internal_clock: i8,
}

impl Display for Lanternfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.internal_clock)
    }
}

impl Lanternfish {
    fn fresh() -> Lanternfish {
        Lanternfish { internal_clock: 8 }
    }

    fn reset(&mut self) {
        self.internal_clock = 6;
    }

    fn pass_day(&mut self) -> Option<Lanternfish> {
        self.internal_clock -= 1;
        if self.internal_clock < 0 {
            self.reset();
            return Some(Lanternfish::fresh());
        }
        return None;
    }
}

impl From<&str> for Lanternfish {
    fn from(s: &str) -> Self {
        Lanternfish {
            internal_clock: s.parse().unwrap(),
        }
    }
}