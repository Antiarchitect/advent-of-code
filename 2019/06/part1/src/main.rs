use std::collections::HashMap;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
struct Body {
    orbits: Option<String>,
}

type Area = HashMap<String, Body>;

fn main() {
    let file = File::open("orbits.txt").expect("Cannot open file");

    let solar_system: Area = BufReader::new(file)
        .lines()
        .fold(HashMap::new(), |mut acc, line| {
            let record: Vec<String> = line
                .expect("Cannot read line")
                .split(')')
                .map(String::from)
                .collect();
            // Parent
            acc.entry(record[0].clone())
                .or_insert(Body { orbits: None });
            // Current
            acc.insert(
                record[1].clone(),
                Body {
                    orbits: Some(record[0].clone()),
                },
            );
            acc
        });
    let total = solar_system.iter().fold(0, |acc, (_name, body)| {
        let mut depth = 0;
        let mut body = Body::clone(body);
        while let Some(newbody) = &body.orbits {
            body = Body::clone(solar_system.get(newbody).expect("Oh shi"));
            depth += 1;
        }
        acc + depth
    });
    println!("{}", total);
}
