use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("passwords.txt").expect("Cannot open file");

    let count = BufReader::new(file)
        .lines()
        .filter_map(|line| {
            let parsed = line.expect("Cannot read line");
            let parts: Vec<&str> = parsed.split_whitespace().collect();
            let indexes: Vec<usize> = parts[0]
                .split("-")
                .map(|x| {
                    x.parse::<usize>()
                        .expect("Cannot parse boundary to positive integer")
                })
                .collect();
            let letter = parts[1].trim_end_matches(':');
            let letters = parts[2]
                .chars()
                .enumerate()
                .filter_map(|(index, item)| {
                    if indexes.iter().any(|&i| i == index + 1) && item.to_string() == letter {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            if letters.len() == 1 {
                Some(parsed)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .len();

    println!("{}", count);
}
