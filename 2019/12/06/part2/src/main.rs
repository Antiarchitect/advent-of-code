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
    let san = solar_system.get("SAN").expect("Oh shi");
    let mut body = Body::clone(san);

    let mut san_parents = Vec::new();
    while let Some(name) = &body.orbits {
        san_parents.push(name.clone());
        body = Body::clone(solar_system.get(name).expect("Oh shi"));
    }

    let you = solar_system.get("YOU").expect("Oh shi");
    let mut body = Body::clone(you);
    let mut you_parents = Vec::new();

    while let Some(name) = &body.orbits {
        let name = name.clone();
        body = Body::clone(solar_system.get(&name).expect("Oh shi"));
        match san_parents.iter().position(|x| *x == name) {
            None => {
                you_parents.push(name.clone());
            }
            Some(index) => {
                println!("{}", index + you_parents.len());
                break;
            }
        }
    }
}
