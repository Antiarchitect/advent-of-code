use std::collections::BTreeSet;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("forms.txt").expect("Cannot open file");

    let answers: usize = BufReader::new(file)
        .lines()
        .fold(vec![BTreeSet::new()], |mut acc, line| {
            let parsed = line.expect("Cannot read line");
            if parsed.trim().is_empty() {
                acc.push(BTreeSet::new());
            } else {
                let last = acc.iter_mut().last().expect("Cannot find last");
                parsed.chars().for_each(|c| {
                    last.insert(c);
                })
            };
            acc
        })
        .iter()
        .map(|set| set.len())
        .sum();

    println!("{}", answers);
}
