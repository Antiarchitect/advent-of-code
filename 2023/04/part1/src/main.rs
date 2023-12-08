use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let result: usize = BufReader::new(file).lines().fold(0, |acc, cardline| {
        let cardline = cardline.expect("Cannot read line");
        let card_parts: Vec<_> = cardline.split(":").collect();
        let card: Vec<_> = card_parts
            .last()
            .expect("No card in line")
            .split(" |")
            .collect();
        let have: HashSet<usize> = HashSet::from_iter(
            card.last()
                .expect("No numbers I have part in card")
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok()),
        );
        let winning: HashSet<usize> = HashSet::from_iter(
            card.first()
                .expect("No winning part in card")
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok()),
        );

        let points =
            winning
                .intersection(&have)
                .fold(0, |acc, _el| if acc == 0 { 1 } else { acc * 2 });
        acc + points
    });

    println!("{result}");
}
