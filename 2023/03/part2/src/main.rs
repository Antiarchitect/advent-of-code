use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

use core::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");

    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.expect("Cannot read line")).collect();
    let arena_h_max_idx: i32 = lines.len() as i32 - 1;
    let arena_l_max_idx: i32 = lines.first().expect("No first line").len() as i32 - 1;

    let mut gear_candidates: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    lines.iter().enumerate().for_each(|(lindex, line)| {
        let mut number_str: (i32, i32, String) = (lindex as i32, 0i32, String::new());
        for (cindex, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if number_str.2.is_empty() {
                    number_str.1 = cindex as i32;
                }
                number_str.2 += &c.to_string();
            };
            if !c.is_digit(10) || (cindex as i32) == arena_l_max_idx {
                if !number_str.2.is_empty() {
                    let number = number_str.2.parse::<usize>().expect("Cannot parse number as usize");
                    for h in max(0, number_str.0 - 1)..=min(arena_h_max_idx, number_str.0 + 1) {
                        let line_chars = lines[h as usize].chars().collect::<Vec<char>>();
                        for l in max(0, number_str.1 - 1)..=min(arena_l_max_idx, number_str.1 + number_str.2.len() as i32) {
                            let candidate = line_chars[l as usize];
                            if candidate == '*'  {
                                let entry = gear_candidates.entry((l, h)).or_insert(Vec::new());
                                entry.push(number);
                            }
                        }
                    }
                    number_str.2 = String::new();
                }
            }
        }
    });

    let result: usize = gear_candidates.iter().fold(0, |acc, ((_l, _h), nums)| {
        if nums.len() == 2 {
            let ratio = nums.first().expect("No first") * nums.last().expect("No last");
            return acc + ratio;
        }
        acc
    });
    println!("{result}");
}
