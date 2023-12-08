use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result: usize = BufReader::new(file).lines().fold(0, |acc, line| {
        let gameline = line.expect("Cannot read line");
        let gameline_parts: Vec<_> = gameline.split(": ").collect();
        let powers = gameline_parts.last().expect("No last part in game").split("; ").fold((0, 0, 0), |mut acc, game| {
            for color_str in game.split(", ") {
                let color_parts: Vec<_> = color_str.split(" ").collect();
                let count = color_parts.first().expect("No first part in color").parse::<usize>().expect("Cannot parse count as usize");
                let color = color_parts.last().expect("No first part in color");
                match *color {
                    "red" => { if count > acc.0 { acc.0 = count } }
                    "green" => { if count > acc.1 { acc.1 = count }}
                    "blue" => { if count > acc.2 { acc.2 = count } }
                    _ => panic!("Unknown color")
                };
            }
            acc
        });
        acc + powers.0 * powers.1 * powers.2
    });
    println!("{result}");
}
