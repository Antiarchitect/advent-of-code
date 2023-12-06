use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result: usize = BufReader::new(file).lines().fold(0, |acc, line| {
        let gameline = line.expect("Cannot read line");
        let gameline_parts: Vec<_> = gameline.split(": ").collect();
        let game_id = gameline_parts.first().expect("No first part in game").replace("Game ", "").parse::<usize>().expect("Cannot parse game id as usize");
        let impossible = gameline_parts.last().expect("No last part in game").split("; ").any(|game| {
            for color_str in game.split(", ") {
                let color_parts: Vec<_> = color_str.split(" ").collect();
                let count = color_parts.first().expect("No first part in color").parse::<usize>().expect("Cannot parse count as usize");
                let color = color_parts.last().expect("No first part in color");
                let impossible = match *color {
                    "red" => { count > 12 }
                    "green" => { count > 13 }
                    "blue" => { count > 14 }
                    _ => panic!("Unknown color")
                };
                if impossible {
                    return true;
                }
            }
            false
        });
        if !impossible {
            return acc + game_id
        }
        acc
    });
    println!("{result}");
}
