use std::collections::{BTreeMap, BTreeSet};

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("bags.txt").expect("Cannot open file");

    let bags: BTreeMap<String, Vec<String>> =
        BufReader::new(file)
            .lines()
            .fold(BTreeMap::new(), |mut acc, line| {
                let bag = line.expect("Cannot read line");
                let dict: Vec<&str> = bag.split(" bags contain ").collect();
                let contains: Vec<String> = match dict[1] {
                    "no other bags." => Vec::new(),
                    _ => dict[1]
                        .split(", ")
                        .map(|x| {
                            x.split(' ')
                                .skip(1)
                                .take(2)
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        })
                        .collect(),
                };
                acc.insert(dict[0].to_string(), contains);
                acc
            });
    let mut containers: BTreeSet<String> = BTreeSet::new();
    containers.insert("shiny gold".to_string());
    loop {
        let containers_filter = containers.clone();
        let count = containers.len();
        bags.iter()
            .filter(|(_bag, contained)| contained.iter().any(|x| containers_filter.contains(x)))
            .for_each(|(bag, _contained)| {
                containers.insert(bag.to_string());
            });
        if containers.len() == count {
            break;
        }
    }
    containers.remove("shiny gold");
    println!("{}", containers.len());
}
