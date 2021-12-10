use std::collections::HashMap;
use std::fmt::Error;
use std::fs;

pub fn day10() {
    let string = fs::read_to_string("bf.txt").unwrap();
    let lines = string.lines();

    let mut bf = Vec::new();
    for (i, line) in lines.enumerate() {
        bf.push(BrainfudgeLine::new(i, line))
    }

    for bfl in bf {
        println!("{:?}", bfl.process())
    }
}

struct BrainfudgeLine<'a> {
    number: usize,
    value: &'a str,
}

impl<'a> BrainfudgeLine<'a> {
    fn new(number:usize, value:&str) -> BrainfudgeLine {
        BrainfudgeLine {
            number,
            value
        }
    }

    fn process(&self) -> Result<(), String> {

    }
}