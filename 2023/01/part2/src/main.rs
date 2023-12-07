use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    static DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let file = File::open("input.txt").expect("Cannot open file");

    let result: usize = BufReader::new(file).lines().fold(0, |acc, line| {
        let line = line.expect("Cannot read line");
        let mut first = 0usize;
        let mut lbuffer = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                first = c.to_digit(10).expect("Cannot parse digit as usize") as usize;
                break;
            }
            lbuffer = lbuffer + &c.to_string();
            for (index, digit) in DIGITS.iter().enumerate() {
                if lbuffer.ends_with(digit) {
                    first = index + 1;
                    break;
                }
            }
            if first > 0 { break; }
        }

        let mut last = 0usize;
        let mut rbuffer = String::new();
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c.to_digit(10).expect("Cannot parse digit as usize") as usize;
                break;
            }
            rbuffer = rbuffer + &c.to_string();
            for (index, digit) in DIGITS.iter().map(|d| d.chars().rev().collect::<String>()).enumerate() {
                if rbuffer.ends_with(&digit) {
                    last = index + 1;
                    break;
                }
            }
            if last > 0 { break; }
        }
        let number = format!("{first}{last}").parse::<usize>().expect("Cannot parse as usize");
        acc + number
    });
    println!("{result}");
}
