use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let rules = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let trees: usize = rules
        .into_iter()
        .map(|(down, right)| {
            let file = File::open("map.txt").expect("Cannot open file");
            // Note that order of `.step_by()` and `.enumerate()` is truly important!
            BufReader::new(file).lines().step_by(down).enumerate().fold(
                0,
                |acc, (line_index, line)| match line
                    .expect("Cannot read the line from the map!")
                    .chars()
                    .cycle()
                    .nth(line_index * right)
                    .expect("Haha! Next char in cycled map does not exist!")
                {
                    '.' => acc,
                    '#' => acc + 1,
                    _ => {
                        panic!("Oh fuck! We found an alien starship!");
                    }
                },
            )
        })
        .product();

    println!("{}", trees);
}
