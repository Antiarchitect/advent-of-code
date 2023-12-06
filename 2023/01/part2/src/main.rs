use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    static DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let file = File::open("input.txt").expect("Cannot open file");

    let result: i32 = BufReader::new(file).lines().fold(0, |acc, line| {
        let line = line.expect("Cannot read line");
        let replaced = line.chars().fold(String::new(), |acc, c| {
            let mut replacement = acc + &c.to_string();
            for (index, digit) in DIGITS.iter().enumerate() {
                replacement = replacement.replace(digit, &(index + 1).to_string());
            }
            replacement
        });
        let filtered: Vec<_> = replaced
        .chars()
        .filter(|i| i.is_digit(10))
        .collect();
        let first = filtered.first().expect("No first digit");
        let last = filtered.last().expect("No last digit");
        let number = format!("{first}{last}").parse::<i32>().expect("Cannot parse to i32");
        println!("{line} {replaced} {number}");
        acc + number
    });
    println!("{result}");
}
