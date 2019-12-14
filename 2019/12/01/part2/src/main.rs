use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("masses.txt").expect("Cannot open file");

    let total = BufReader::new(file).lines().fold(0u64, |mut acc, line| {
        let mut mass = line
            .expect("Cannot read line")
            .parse::<u64>()
            .expect("Cannot parse positive integer");
        loop {
            let step_mass: i64 = mass as i64 / 3 - 2;
            if step_mass <= 0 { break; }
            acc += step_mass as u64;
            mass = step_mass as u64;
        }
        acc
    });
    println!("{}", total);
}
