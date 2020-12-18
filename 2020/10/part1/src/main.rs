use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("adapters.txt").expect("Cannot open file");

    let mut adapters: Vec<u64> = BufReader::new(file)
        .lines()
        .map(|x| {
            x.expect("Cannot read line")
                .parse()
                .expect("Cannot parse number")
        })
        .collect();
    adapters.push(0);
    adapters.push(adapters.iter().max().expect("No max adapter") + 3);
    adapters.sort_unstable();
    let diff = &adapters[..].windows(2).fold((0, 0), |mut acc, pair| {
        match pair[1] - pair[0] {
            1 => acc.0 += 1,
            3 => acc.1 += 1,
            _ => {}
        }
        acc
    });
    println!("{}", diff.0 * diff.1);
}
