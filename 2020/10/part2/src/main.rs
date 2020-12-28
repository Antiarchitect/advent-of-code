use std::collections::BTreeMap;
use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("adapters.txt").expect("Cannot open file");

    let mut adapters: Vec<usize> = BufReader::new(file)
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
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    counts.insert(0, 1);
    let ways = adapters.iter().skip(1).fold(counts, |mut acc, e| {
        let start = if *e < 3 { 0 } else { *e - 3 };
        acc.insert(*e, (start..*e).filter_map(|i| acc.get(&i)).sum());
        acc
    });
    let (_k, value) = ways.iter().last().expect("No last element");
    println!("{:?}", value);
}
