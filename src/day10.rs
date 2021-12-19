use std::collections::{BinaryHeap, HashMap};
use std::fmt::Error;
use std::fs;

pub fn day10() {
    let string = fs::read_to_string("bf.txt").unwrap();
    let lines = string.lines();

    let mut total_error_score = 0;
    let mut incomplete_scores = BinaryHeap::new();
    for line in lines {
        match check_line(line) {
            Corrupted(n) => total_error_score += n,
            Incomplete(n) => incomplete_scores.push(n),
        }
    }
    println!("Total syntax error score: {}", total_error_score);

    for _ in 0..(incomplete_scores.len()/2) {
        incomplete_scores.pop();
    }
    println!("Middle score of incompletes: {}", incomplete_scores.pop().unwrap())
}

fn check_line(line: &str) -> LineStatus {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if OPENING_CHARS.contains(&c) {
            stack.push(c);
        } else {
            if let Some(opening_char) = stack.pop() {
                if chars_match(opening_char, c) {
                    continue
                }
            } //else
            return Corrupted(score_corrupt(c))
        }
    }
    //Incomplete, base score on completion string
    let mut completion_string = Vec::new();
    while !stack.is_empty() {
        completion_string.push(stack.pop().unwrap())
    }
    Incomplete(score_incomplete(completion_string))
}


static OPENING_CHARS: [char; 4] = ['(','[','{','<'];

fn chars_match(opening: char, closing: char) -> bool {
    match (opening, closing) {
        ('(',')') |
        ('[',']') |
        ('{','}') |
        ('<','>') => true,
        _ => false
    }
}
fn score_corrupt(closing_char: char) -> u32 {
    match closing_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected char!")
    }
}
fn score_incomplete(completion_string: Vec<char>) -> u64 {
    let mut score = 0;
    for c in completion_string {
        score *= 5;
        match c { //Characters are open, not closed as would be expected, see usage at end of check_line for why
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => (),
        }
    }
    score
}

enum LineStatus {
    Corrupted(u32),
    Incomplete(u64),
}
use LineStatus::*;