use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let lines: Vec<String> = BufReader::new(file).lines().map(|i| i.expect("Cannot read line")).collect();
    let result: usize = lines.split(|i| i.is_empty()).map(|i| i.iter().map(|i| i.parse::<usize>().expect("Not a number")).sum()).max().expect("No max");
    println!("{}", result);
}
