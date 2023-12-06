use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result: i32 = BufReader::new(file).lines().fold(0, |acc, line| {
        let filtered: Vec<_> = line.expect("Cannot read line").chars().filter(|i| i.is_digit(10)).collect();
        let first = filtered.first().expect("No first digit");
        let last = filtered.last().expect("No last digit");
        let number = format!("{first}{last}").parse::<i32>().expect("Cannot parse to i32");
        acc + number
    });
    println!("{}", result);
}
