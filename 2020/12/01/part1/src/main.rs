use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("report.txt").expect("Cannot open file");

    let items: Vec<i64> = BufReader::new(file).lines().map(|line| {
        line
            .expect("Cannot read line")
            .parse::<i64>()
            .expect("Cannot parse integer")
    }).collect();

    let matches: Vec<i64> = items.iter().cloned().filter(|item| items.contains(&(2020 - item))).collect();
    let first = matches[0];

    println!("{}", first * (2020 - first));
}
