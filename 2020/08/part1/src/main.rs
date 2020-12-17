use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

struct Instruction {
    fresh: bool,
    verb: String,
    arg: i64,
}

fn main() {
    let file = File::open("instructions.txt").expect("Cannot open file");

    let mut instructions: Vec<Instruction> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.expect("Cannot read line");
            let mut splitter = line.split(' ');
            Instruction {
                fresh: true,
                verb: splitter.next().expect("No verb").to_string(),
                arg: splitter
                    .next()
                    .expect("No arg")
                    .parse()
                    .expect("Cannot parse arg"),
            }
        })
        .collect();

    let acc = run(&mut 0usize, &mut 0i64, &mut instructions);
    println!("{}", acc);
}

fn run(pointer: &mut usize, acc: &mut i64, instructions: &mut [Instruction]) -> i64 {
    let mut instruction = &mut instructions[*pointer];
    if !instruction.fresh {
        return acc.to_owned();
    };
    instruction.fresh = false;
    match instruction.verb.as_ref() {
        "acc" => {
            *acc += instruction.arg;
            *pointer += 1;
        }
        "jmp" => {
            *pointer = (*pointer as i64 + instruction.arg) as usize;
        }
        "nop" => {
            *pointer += 1;
        }
        _ => {
            panic!("Unknown instruction detected!")
        }
    }
    run(pointer, acc, instructions)
}
