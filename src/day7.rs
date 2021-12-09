use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::Hash;

pub fn day7() {
    let string = fs::read_to_string("crabz.txt").unwrap();
    let crabz: Vec<i32> = string.split(',').map(|x| x.parse().unwrap()).collect();

    let lower_bound = crabz.iter().min().unwrap().clone();
    let higher_bound = crabz.iter().max().unwrap().clone();

    // position -> amount of crabz
    let mut crabz_counts: BTreeMap<i32, i32> = BTreeMap::new();

    for crab in &crabz {
        *crabz_counts.entry(*crab).or_insert(0) += 1;
    }

    for position in 0..higher_bound {
        crabz_counts.entry(position).or_insert(0);
    }

    let mut costs: Vec<(i32, i32)> = Vec::new();
    for (to, _) in &crabz_counts {
        let mut cost:i32 = 0;
        for (from, from_crabz)  in &crabz_counts {
            let n = (from - to).abs();
            cost += (n*(n+1) * from_crabz)/2;
        }
        costs.push((*to as i32, cost as i32));
    }

    for (pos, crab_count) in crabz_counts {
        println!("POS:{} \t-> CRABZ:{}", pos, crab_count);
    }

    costs.sort_by_key(|a| a.1);
    for (to, cost) in costs {
        println!("TO:{} -> COST:{} ", to, cost);
    }
}
