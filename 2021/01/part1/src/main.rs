use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result: (usize, Option<u64>) = BufReader::new(file).lines().fold((0, None), |acc, line| {
        let current = line
            .expect("Cannot read line")
            .parse::<u64>()
            .expect("Cannot parse positive integer");
        if let Some(previous) = acc.1 {
            if current > previous {
                return (acc.0 + 1, Some(current));
            }
        };
        (acc.0, Some(current))
    });

    println!("{}", result.0);
}
