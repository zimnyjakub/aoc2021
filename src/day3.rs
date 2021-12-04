use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Not;

pub fn day3() {
    println!("day3");

    let string = fs::read_to_string("bleepbloop.txt").unwrap();
    let mut lines = string.lines();

    let mut data: Vec<isize> = Vec::new();
    let data_len = 12;
    for line in lines {
        let intval = isize::from_str_radix(line, 2).unwrap();
        data.push(intval);
    }

    //
    // for row in &data {
    //     println!("{:#016b}", row);
    // }

    let mut gamma = 0;
    let mut epsilon = 0;
    for n in 0..data_len {
        let mut c = 0;
        for row in &data {
            match row >> n & 1 {
                1 => c += 1,
                0 => c -= 1,
                _ => ()
            }
        }
        let most_common: i32 = if c >= 0 { 1 } else { 0 };
        println!("pos: {} -> {}", n, most_common);
        gamma = gamma | ((1 & most_common) << n);
        gamma.reverse_bits();
        epsilon = epsilon | ((1 ^ most_common) << n);
        epsilon.reverse_bits();
    }

    println!("gamma: {}, {:012b}", gamma, gamma);
    println!("epsilon: {}, {:012b}", epsilon, epsilon);
    println!("aoc solution for day 3 part 1 {}", gamma * epsilon);

    let mut oxygen_gen_rating = 0;
    let mut co2_scrubber_rating = 0;
    for row in &data {
        if gamma & *row as i32 == 1 {
            oxygen_gen_rating= row.clone();
        }

        if epsilon & *row as i32 == 1 {
            co2_scrubber_rating = row.clone();
        }
    }
    println!("oxygen generator rating: {} ({:012b})", oxygen_gen_rating, oxygen_gen_rating);
    println!("co2 scrubber rating: {} ({:012b})", co2_scrubber_rating, co2_scrubber_rating)
}



