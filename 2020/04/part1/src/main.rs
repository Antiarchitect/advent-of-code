use std::collections::BTreeMap;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let file = File::open("passports.txt").expect("Cannot open file");

    let passports: Vec<BTreeMap<String, String>> =
        BufReader::new(file)
            .lines()
            .fold(vec![BTreeMap::new()], |mut acc, line| {
                let parsed = line.expect("Cannot read line");
                if parsed.trim().is_empty() {
                    acc.push(BTreeMap::new());
                } else {
                    let last = acc.iter_mut().last().expect("Cannot find last");
                    let mut new: BTreeMap<String, String> =
                        parsed
                            .split_whitespace()
                            .fold(BTreeMap::new(), |mut acc, entry| {
                                let kv: Vec<&str> = entry.split(':').collect();
                                acc.entry(kv[0].to_string())
                                    .or_insert_with(|| kv[1].to_string());
                                acc
                            });
                    last.append(&mut new);
                };
                acc
            });

    let valid = passports
        .iter()
        .filter(|&passport| {
            REQUIRED_FIELDS
                .iter()
                .all(|&field| passport.contains_key(field))
        })
        .count();

    println!("{}", valid);
}
