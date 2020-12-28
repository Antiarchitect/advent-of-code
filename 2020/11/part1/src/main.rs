use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("seatmap.txt").expect("Cannot open file");

    let seatmap: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Cannot read line").chars().collect())
        .collect();

    let last_row_index = seatmap.len() - 1;
    let last_col_index = seatmap.first().expect("No rows in seatmap").len() - 1;

    let mut initial = seatmap;

    loop {
        let resulting = initial
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_index, &col)| {
                        if col == '.' {
                            return '.';
                        }
                        let row_start = if row_index == 0 { 0 } else { row_index - 1 };
                        let row_end = if row_index == last_row_index {
                            last_row_index
                        } else {
                            row_index + 1
                        };
                        let col_start = if col_index == 0 { 0 } else { col_index - 1 };
                        let col_end = if col_index == last_col_index {
                            last_col_index
                        } else {
                            col_index + 1
                        };
                        let occupied = initial
                            .iter()
                            .enumerate()
                            .take(row_end + 1)
                            .skip(row_start)
                            .fold(0, |acc, (row, row_content)| {
                                acc + row_content
                                    .iter()
                                    .enumerate()
                                    .take(col_end + 1)
                                    .skip(col_start)
                                    .fold(0, |acc, (col, &col_content)| {
                                        if row == row_index && col == col_index {
                                            return acc;
                                        };
                                        if col_content == '#' {
                                            return acc + 1;
                                        };
                                        acc
                                    })
                            });
                        if col == 'L' && occupied == 0 {
                            return '#';
                        };
                        if col == '#' && occupied >= 4 {
                            return 'L';
                        };
                        col
                    })
                    .collect()
            })
            .collect();
        if initial == resulting {
            break;
        } else {
            initial = resulting
        };
    }
    println!(
        "{}",
        initial.iter().flatten().filter(|&x| *x == '#').count()
    );
}
