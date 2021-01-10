use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("schedule.txt").expect("Cannot open file");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Cannot read line"));
    let _timestamp = lines.next().expect("No first line");
    let schedule: Vec<Option<usize>> = lines
        .next()
        .expect("No second line")
        .split(',')
        .map(|x| match x {
            "x" => None,
            value => Some(value.parse().expect("Cannot parse as positive integer")),
        })
        .collect();
    let mut step = 1;
    let mut start = 1;
    schedule.iter().enumerate().for_each(|(shift, bus)| {
        for timestamp in (start..).step_by(step) {
            match bus {
                None => {
                    break;
                }
                Some(bus) => {
                    if (timestamp + shift) % bus == 0 {
                        step *= bus;
                        start = timestamp;
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
    });
    println!("{:#?}", start);
}
