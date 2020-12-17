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

    let acc = run(
        &mut 0usize,
        &mut 0usize,
        &mut 0usize,
        &mut 0i64,
        &mut instructions,
    );
    println!("{}", acc);
}

fn run(
    mod_counter: &mut usize,
    mod_pointer: &mut usize,
    pointer: &mut usize,
    acc: &mut i64,
    instructions: &mut [Instruction],
) -> i64 {
    let mut instruction = &mut instructions[*pointer];
    if !instruction.fresh {
        *mod_counter += 1;
        instructions
            .iter_mut()
            .for_each(|i: &mut Instruction| i.fresh = true);
        run(
            mod_counter,
            &mut 0usize,
            &mut 0usize,
            &mut 0i64,
            instructions,
        )
    } else {
        match instruction.verb.as_ref() {
            "acc" => {
                *acc += instruction.arg;
                *pointer += 1;
            }
            "jmp" => {
                if *mod_pointer == *mod_counter {
                    *pointer += 1;
                } else {
                    *pointer = (*pointer as i64 + instruction.arg) as usize;
                }
                *mod_pointer += 1;
            }
            "nop" => {
                if *mod_pointer == *mod_counter {
                    *pointer = (*pointer as i64 + instruction.arg) as usize;
                } else {
                    *pointer += 1;
                }
                *mod_pointer += 1;
            }
            _ => {
                panic!("Unknown instruction detected!")
            }
        }
        instruction.fresh = false;
        if instructions.iter_mut().count() <= *pointer {
            return *acc;
        }
        run(mod_counter, mod_pointer, pointer, acc, instructions)
    }
}
