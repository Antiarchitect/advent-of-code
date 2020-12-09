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
            let boundary: Vec<usize> = parts[0]
                .split("-")
                .map(|x| {
                    x.parse::<usize>()
                        .expect("Cannot parse boundary to positive integer")
                })
                .collect();
            let letter = parts[1].trim_end_matches(':');
            let letters_count = parts[2]
                .chars()
                .filter(|char| char.to_string() == letter)
                .collect::<Vec<char>>()
                .len();
            if letters_count >= boundary[0] && letters_count <= boundary[1] {
                Some(parsed)
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .len();

    println!("{}", count);
}
