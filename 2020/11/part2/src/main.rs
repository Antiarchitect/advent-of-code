use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("seatmap.txt").expect("Cannot open file");

    let seatmap: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("Cannot read line").chars().collect())
        .collect();

    let row_end = seatmap.len() - 1;
    let col_end = seatmap.first().expect("No rows in seatmap").len() - 1;

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
                        let mut occupied = 0;
                        // Left
                        for i in 1.. {
                            let row_in = row_index as i32;
                            let col_in = col_index as i32 - i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Right
                        for i in 1.. {
                            let row_in = row_index as i32;
                            let col_in = col_index as i32 + i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Top
                        for i in 1.. {
                            let row_in = row_index as i32 - i;
                            let col_in = col_index as i32;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Bottom
                        for i in 1.. {
                            let row_in = row_index as i32 + i;
                            let col_in = col_index as i32;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Topleft
                        for i in 1.. {
                            let row_in = row_index as i32 - i;
                            let col_in = col_index as i32 - i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Topright
                        for i in 1.. {
                            let row_in = row_index as i32 - i;
                            let col_in = col_index as i32 + i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Bottomleft
                        for i in 1.. {
                            let row_in = row_index as i32 + i;
                            let col_in = col_index as i32 - i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }
                        // Bottomright
                        for i in 1.. {
                            let row_in = row_index as i32 + i;
                            let col_in = col_index as i32 + i;
                            if row_in < 0
                                || row_in > row_end as i32
                                || col_in < 0
                                || col_in > col_end as i32
                            {
                                break;
                            }
                            let place = initial[row_in as usize][col_in as usize];
                            if place == '.' {
                                continue;
                            };
                            if place == 'L' {
                                break;
                            }
                            if place == '#' {
                                occupied += 1;
                                break;
                            }
                        }

                        if col == 'L' && occupied == 0 {
                            return '#';
                        };
                        if col == '#' && occupied >= 5 {
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
