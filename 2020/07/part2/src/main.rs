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
                let dict: Vec<&str> = bag.split(" bags contain ").collect();
                let contains: BTreeMap<String, usize> = match dict[1] {
                    "no other bags." => BTreeMap::new(),
                    _ => dict[1]
                        .split(", ")
                        .map(|x| {
                            let splitted: Vec<String> =
                                x.split(' ').map(|x| x.to_string()).collect();
                            (
                                [splitted[1].to_string(), splitted[2].to_string()].join(" "),
                                splitted[0]
                                    .parse::<usize>()
                                    .expect("Cannot parse bags count"),
                            )
                        })
                        .collect::<BTreeMap<String, usize>>(),
                };
                acc.insert(dict[0].to_string(), contains);
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
