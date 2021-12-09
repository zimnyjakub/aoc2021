use std::collections::{BTreeMap, HashMap};
use std::fs;

pub fn day9() {
    let string = fs::read_to_string("lavalamp.txt").unwrap();
    let lines= string.lines();

    let rows: Vec<&str> = lines.collect();

    let mut height_map: BTreeMap<(i32, i32), i32> = BTreeMap::new();

    for y in 0..rows.len() {
        let xes = rows[y].chars();
        for (x, value) in xes.enumerate() {
            height_map.insert((x as i32,y as i32), value.to_string().parse().unwrap());
        }
    }

    for p in &height_map {
        println!("{:?}", p);
    }

    let mut low_points: Vec<(i32,i32,i32)> = Vec::new();
    for p in &height_map {
        println!("{:?}", p);
    }

}

