use std::collections::BTreeMap;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("bags.txt").expect("Cannot open file");

    let bags: BTreeMap<String, BTreeMap<String, usize>> =
        BufReader::new(file)
            .lines()
            .fold(BTreeMap::new(), |mut acc, line| {
                let bag = line.expect("Cannot read line");
                let mut splitter = bag.split(" bags contain ");
                let name = splitter.next().expect("No bag name").to_string();
                let contains: BTreeMap<String, usize> =
                    match splitter.next().expect("No contained bags info") {
                        "no other bags." => BTreeMap::new(),
                        info => info
                            .split(", ")
                            .map(|x| {
                                let mut splitter = x.split(' ');
                                let count = splitter
                                    .next()
                                    .expect("No bags count")
                                    .parse::<usize>()
                                    .expect("Cannot parse bags count");
                                let name = splitter
                                    .take(2)
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(" ");
                                (name, count)
                            })
                            .collect::<BTreeMap<String, usize>>(),
                    };
                acc.insert(name, contains);
                acc
            });
    println!("{}", nested_sum(&bags, "shiny gold"));
}

fn nested_sum(bags: &BTreeMap<String, BTreeMap<String, usize>>, key: &str) -> usize {
    bags.get(key)
        .expect("Could not find key")
        .iter()
        .map(|(key, value)| value * (1 + nested_sum(&bags, key)))
        .sum()
}
