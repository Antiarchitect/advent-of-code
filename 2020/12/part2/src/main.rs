use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

struct Position {
    east: i64,
    north: i64,
}

struct Waypoint {
    east: i64,
    north: i64,
}

struct Map {
    position: Position,
    waypoint: Waypoint,
}

fn main() {
    let file = File::open("navigation.txt").expect("Cannot open file");

    let map: Map = BufReader::new(file).lines().fold(
        Map {
            position: Position { east: 0, north: 0 },
            waypoint: Waypoint { east: 10, north: 1 },
        },
        |mut acc, line| {
            let line = line.expect("Cannot read line");
            let (instruction, value) = line.split_at(1);
            let value = value
                .parse::<i64>()
                .expect("Cannot parse instruction value");
            match instruction {
                "N" => {
                    acc.waypoint.north += value;
                }
                "S" => {
                    acc.waypoint.north -= value;
                }
                "E" => {
                    acc.waypoint.east += value;
                }
                "W" => {
                    acc.waypoint.east -= value;
                }
                "L" => {
                    for _ in 0..(value / 90) {
                        let new_east = -acc.waypoint.north;
                        let new_north = acc.waypoint.east;
                        acc.waypoint.east = new_east;
                        acc.waypoint.north = new_north;
                    }
                }
                "R" => {
                    for _ in 0..(value / 90) {
                        let new_east = acc.waypoint.north;
                        let new_north = -acc.waypoint.east;
                        acc.waypoint.east = new_east;
                        acc.waypoint.north = new_north;
                    }
                }
                "F" => {
                    acc.position.east += acc.waypoint.east * value;
                    acc.position.north += acc.waypoint.north * value;
                }
                _ => {
                    panic!("Unknown instruction met!")
                }
            }
            acc
        },
    );
    println!("{}", map.position.east.abs() + map.position.north.abs());
}
