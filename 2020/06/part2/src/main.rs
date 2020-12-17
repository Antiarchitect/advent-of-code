use std::collections::BTreeSet;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("forms.txt").expect("Cannot open file");

    let answers: usize = BufReader::new(file)
        .lines()
        .fold(vec![vec![]], |mut acc, line| {
            let parsed = line.expect("Cannot read line");
            if parsed.trim().is_empty() {
                acc.push(Vec::new());
            } else {
                let last = acc.iter_mut().last().expect("Cannot find last");
                let new: BTreeSet<char> = parsed.chars().collect();
                last.push(new);
            };
            acc
        })
        .iter()
        .map(|sets| {
            let superset = sets.iter().fold(BTreeSet::new(), |mut acc, set| {
                set.iter().for_each(|c| {
                    acc.insert(c);
                });
                acc
            });
            superset
                .iter()
                .filter(|c| sets.iter().all(|set| set.contains(c)))
                .count()
        })
        .sum();

    println!("{}", answers);
}
