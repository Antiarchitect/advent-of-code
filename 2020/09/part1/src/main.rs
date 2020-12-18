use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("stream.txt").expect("Cannot open file");

    let preamble_len = 25;

    let numbers: Vec<u64> = BufReader::new(file)
        .lines()
        .map(|x| {
            x.expect("Cannot read line")
                .parse()
                .expect("Cannot parse number")
        })
        .collect();
    let mut shift = 0;
    loop {
        let slice = &numbers[shift..(preamble_len + shift)];
        let next = &numbers[preamble_len + shift];
        let valid = slice[0..preamble_len]
            .iter()
            .enumerate()
            .fold(Vec::new(), |acc, (i, first)| {
                slice[0..preamble_len]
                    .iter()
                    .enumerate()
                    .fold(acc, |mut acc, (j, second)| {
                        if j != i {
                            acc.push(first + second)
                        }
                        acc
                    })
            })
            .iter()
            .any(|&x| *next == x);
        if !valid {
            println!("{}", next);
            break;
        }
        shift += 1;
    }
}
