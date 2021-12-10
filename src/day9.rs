use std::collections::{BTreeMap, HashMap};
use std::fs;

pub fn day9() {
    let string = fs::read_to_string("lavalamp.txt").unwrap();
    let lines = string.lines();

    let rows: Vec<&str> = lines.collect();

    let mut height_map: BTreeMap<(i32, i32), i32> = BTreeMap::new();

    for y in 0..rows.len() {
        let xes = rows[y].chars();
        for (x, value) in xes.enumerate() {
            height_map.insert((x as i32, y as i32), value.to_string().parse().unwrap());
        }
    }

    let mut risk_levels: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    for ((x, y), v) in &height_map {
        let up = height_map.get(&(*x, *y - 1));
        let down = height_map.get(&(*x, *y + 1));
        let right = height_map.get(&(*x + 1, *y));
        let left = height_map.get(&(*x - 1, *y));

        if up.is_some() && up.unwrap() <= v {
            continue;
        }
        if down.is_some() && down.unwrap() <= v {
            continue;
        }
        if left.is_some() && left.unwrap() <= v {
            continue;
        }
        if right.is_some() && right.unwrap() <= v {
            continue;
        }

        risk_levels.insert((*x, *y), *v+1);
    }

    for p in &risk_levels {
        println!("{:?}", p)
    }

    let aoc: i32 = risk_levels.iter().map(|it| it.1).sum();

    println!("aoc ans: {}", aoc);
}

