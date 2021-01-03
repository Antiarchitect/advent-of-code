use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

struct Position {
    east: i64,
    north: i64,
    facing: i64,
}

fn main() {
    let file = File::open("navigation.txt").expect("Cannot open file");

    let position: Position = BufReader::new(file).lines().fold(
        Position {
            east: 0,
            north: 0,
            facing: 90,
        },
        |mut acc, line| {
            let line = line.expect("Cannot read line");
            let (instruction, value) = line.split_at(1);
            let value = value
                .parse::<i64>()
                .expect("Cannot parse instruction value");
            match instruction {
                "N" => {
                    acc.north += value;
                }
                "S" => {
                    acc.north -= value;
                }
                "E" => {
                    acc.east += value;
                }
                "W" => {
                    acc.east -= value;
                }
                "L" => {
                    let mut new = acc.facing - value;
                    if new < 0 {
                        new += 360;
                    }
                    acc.facing = new;
                }
                "R" => {
                    let mut new = acc.facing + value;
                    if new >= 360 {
                        new -= 360;
                    }
                    acc.facing = new;
                }
                "F" => match acc.facing {
                    90 => {
                        acc.east += value;
                    }
                    270 => {
                        acc.east -= value;
                    }
                    0 => {
                        acc.north += value;
                    }
                    180 => {
                        acc.north -= value;
                    }
                    _ => {
                        panic!("Cannot forward by diagonal!")
                    }
                },
                _ => {
                    panic!("Unknown instruction met!")
                }
            }
            acc
        },
    );
    println!("{}", position.east.abs() + position.north.abs());
}
