use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

struct Position {
    step: usize,
    index: usize,
}

fn main() {
    let file = File::open("boardingpasses.txt").expect("Cannot open file");

    let max_id = BufReader::new(file)
        .lines()
        .map(|line| {
            let codeline = line
                .expect("Cannot read line")
                .chars()
                .collect::<Vec<char>>();
            let (rowcode, columncode) = codeline.split_at(7);
            let row = rowcode
                .iter()
                .fold(
                    Position {
                        step: 128,
                        index: 0,
                    },
                    |mut acc, code| {
                        acc.step /= 2;
                        match code {
                            'B' => acc.index += acc.step,
                            'F' => {}
                            _ => {
                                panic!("Wrong rowcode!")
                            }
                        };
                        acc
                    },
                )
                .index;
            let column = columncode
                .iter()
                .fold(Position { step: 8, index: 0 }, |mut acc, code| {
                    acc.step /= 2;
                    match code {
                        'R' => acc.index += acc.step,
                        'L' => {}
                        _ => {
                            panic!("Wrong columncode!")
                        }
                    };
                    acc
                })
                .index;
            row * 8 + column
        })
        .max()
        .expect("No max seat id!");

    println!("{}", max_id);
}
