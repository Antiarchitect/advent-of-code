use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("report.txt").expect("Cannot open file");

    let items: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.expect("Cannot read line")
                .parse::<i64>()
                .expect("Cannot parse integer")
        })
        .collect();

    let len = items.len();

    for first in 0..len {
        for second in (first + 1)..len {
            for third in (second + 1)..len {
                if items[first] + items[second] + items[third] == 2020 {
                    println!("{}", items[first] * items[second] * items[third]);
                }
            }
        }
    }
}
