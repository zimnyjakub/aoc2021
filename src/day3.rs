use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Not;

pub fn day3() {
    println!("day3");

    let string = fs::read_to_string("bleepbloop.txt").unwrap();
    let mut lines = string.lines();

    let mut data: Vec<isize> = Vec::new();
    let data_len = 11;
    for line in lines {
        let intval = isize::from_str_radix(line, 2).unwrap();
        data.push(intval);
    }

    //
    // for row in &data {
    //     println!("{:#016b}", row);
    // }

    let mut most_common_mask = 0;
    for n in 0..data_len {
        let mut c = 0;
        for row in &data {
            match row >> n & 1 {
                1 => c +=1,
                0 => c -=1,
                _ => ()
            }
        }
        let most_common: i32 = if c >= 0 { 1 } else { 0 };
        println!("pos: {} -> {}", n, most_common);
        most_common_mask = most_common_mask | ((1 & most_common) << n);
        most_common_mask.reverse_bits();
    }

    println!("gamma: {}, {:b}", most_common_mask, most_common_mask)
}



