use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

struct State {
    start: usize,
    step: usize,
}

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
    let state =
        schedule
            .iter()
            .enumerate()
            .fold(State { start: 1, step: 1 }, |mut acc, (shift, bus)| {
                for timestamp in (acc.start..).step_by(acc.step) {
                    match bus {
                        None => {
                            break;
                        }
                        Some(bus) => {
                            if (timestamp + shift) % bus == 0 {
                                acc.step *= bus;
                                acc.start = timestamp;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                }
                acc
            });
    println!("{:#?}", state.start);
}
