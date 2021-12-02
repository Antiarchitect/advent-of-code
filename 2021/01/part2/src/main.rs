use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let lines: Vec<u64> = BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("Cannot read line")
                .parse::<u64>()
                .expect("Cannot parse positive integer")
        })
        .collect();
    let result: (usize, Option<u64>) = lines.as_slice().windows(3).fold((0, None), |acc, depth| {
        let current = depth.iter().sum();
        if let Some(previous) = acc.1 {
            if current > previous {
                return (acc.0 + 1, Some(current));
            }
        };
        (acc.0, Some(current))
    });

    println!("{}", result.0);
}
