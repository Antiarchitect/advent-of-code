use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("map.txt").expect("Cannot open file");

    let trees = BufReader::new(file)
        .lines()
        .enumerate()
        .fold(0, |acc, (line_index, line)| {
            match line
                .expect("Cannot read the line from the map!")
                .chars()
                .cycle()
                .nth(line_index * 3)
                .expect("Haha! Next char in cycled map does not exist!")
            {
                '.' => acc,
                '#' => acc + 1,
                _ => {
                    panic!("Oh fuck! We found an alien starship!");
                }
            }
        });

    println!("{}", trees);
}
