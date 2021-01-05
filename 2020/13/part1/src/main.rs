use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("schedule.txt").expect("Cannot open file");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Cannot read line"));
    let timestamp: usize = lines
        .next()
        .expect("No first line")
        .parse()
        .expect("Cannot parse as positive integer");
    let bus: usize = lines
        .next()
        .expect("No second line")
        .split(',')
        .filter_map(|x| match x {
            "x" => None,
            value => Some(value.parse().expect("Cannot parse as positive integer")),
        })
        .min_by_key(|x| (timestamp / x + 1) * x - timestamp)
        .expect("No earliest bus");

    println!("{:#?}", bus * ((timestamp / bus + 1) * bus - timestamp));
}
