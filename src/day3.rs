use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Not;

pub fn day3() {
    println!("day3");

    let string = fs::read_to_string("bleepbloop.txt").unwrap();
    let mut lines = string.lines();

    let mut data: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            match c {
                '1' => row.push(true),
                '0' => row.push(false),
                _ => ()
            }
        }
        data.push(row);
    }

    let mut diag = BinaryDiagnostic::new(data);
    diag.compute_epsilon();
    diag.compute_gamma_from_epsilon();

    println!("{}", diag)
}

#[derive(Debug)]
struct BinaryDiagnostic {
    gamma: Vec<bool>,
    epsilon: Vec<bool>,

    input: Vec<Vec<bool>>,
}


impl Display for BinaryDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "epsilon: {} | gamma: {}", bool_vec_to_string(&self.epsilon), bool_vec_to_string(&self.gamma))
    }
}

fn bool_vec_to_string(input: &Vec<bool>) -> String {
    let mut s = String::with_capacity(input.len());
    for e in input {
        match e {
            true => s.push('1'),
            false => s.push('0')
        }
    };
    s
}

impl BinaryDiagnostic {
    fn new(input: Vec<Vec<bool>>) -> BinaryDiagnostic{
        BinaryDiagnostic {
            epsilon: Vec::new(),
            gamma: Vec::new(),

            input
        }
    }

    fn compute_epsilon(&mut self) {
        let len = self.input.get(0).unwrap().len();
        for i in 0..len {
            let mut c = 0;
            for row in &self.input {
                if row[i] {
                    c +=1;
                } else {
                    c -=1;
                }
            }

            if c > 0 {
                self.epsilon.push(true);
            } else {
                self.epsilon.push(false);
            }
        }
    }

    fn compute_gamma_from_epsilon(&mut self) {
        for eps in &self.epsilon {
            self.gamma.push(eps.clone().not());
        }
    }
}



