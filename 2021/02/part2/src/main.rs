use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

#[derive(Default)]
struct Position {
    aim: i64,
    depth: i64,
    distance: i64,
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result = BufReader::new(file)
        .lines()
        .fold(Position::default(), |mut acc, line| {
            let line = line.expect("Cannot read line");
            let mut splitted = line.split_whitespace();
            let direction = splitted.next().expect("Command has no the direction part");
            let value = splitted
                .next()
                .expect("Command has no the value part")
                .parse::<i64>()
                .expect("Cannot parse value as integer");
            match direction {
                "forward" => {
                    acc.distance += value;
                    acc.depth += acc.aim * value;
                }
                "up" => {
                    acc.aim -= value;
                }
                "down" => {
                    acc.aim += value;
                }
                _ => {
                    panic!("Unknown command received!")
                }
            }
            acc
        });

    println!("{}", result.depth * result.distance);
}
