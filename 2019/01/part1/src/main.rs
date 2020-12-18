use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("masses.txt").expect("Cannot open file");

    let total = BufReader::new(file).lines().fold(0u64, |acc, line| {
        let fuel = line
            .expect("Cannot read line")
            .parse::<u64>()
            .expect("Cannot parse positive integer")
            / 3
            - 2;
        acc + fuel
    });
    println!("{}", total);
}
