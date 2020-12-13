use std::collections::BTreeMap;

use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;

use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref H_COLOR_REGEX: Regex = Regex::new(r"\A#[0-9a-f]{6}\z").unwrap();
    static ref E_COLOR_REGEX: Regex = Regex::new(r"\A(amb|blu|brn|gry|grn|hzl|oth)\z").unwrap();
    static ref PASSPORT_REGEX: Regex = Regex::new(r"\A\d{9}\z").unwrap();
}

fn main() {
    let file = File::open("passports.txt").expect("Cannot open file");

    let passports: Vec<BTreeMap<String, String>> =
        BufReader::new(file)
            .lines()
            .fold(vec![BTreeMap::new()], |mut acc, line| {
                let parsed = line.expect("Cannot read line");
                if parsed.trim().is_empty() {
                    acc.push(BTreeMap::new());
                } else {
                    let last = acc.iter_mut().last().expect("Cannot find last");
                    let mut new: BTreeMap<String, String> =
                        parsed
                            .split_whitespace()
                            .fold(BTreeMap::new(), |mut acc, entry| {
                                let kv: Vec<&str> = entry.split(':').collect();
                                acc.entry(kv[0].to_string())
                                    .or_insert_with(|| kv[1].to_string());
                                acc
                            });
                    last.append(&mut new);
                };
                acc
            });

    let valid = passports
        .iter()
        .filter(|&passport| {
            match passport.get("byr") {
                None => return false,
                Some(string) => match string.parse::<u32>() {
                    Err(_) => return false,
                    Ok(value) => {
                        if !(value >= 1920 && value <= 2002) {
                            return false;
                        }
                    }
                },
            };
            match passport.get("iyr") {
                None => return false,
                Some(string) => match string.parse::<u32>() {
                    Err(_) => return false,
                    Ok(value) => {
                        if !(value >= 2010 && value <= 2020) {
                            return false;
                        }
                    }
                },
            };
            match passport.get("eyr") {
                None => return false,
                Some(string) => match string.parse::<u32>() {
                    Err(_) => return false,
                    Ok(value) => {
                        if !(value >= 2020 && value <= 2030) {
                            return false;
                        }
                    }
                },
            };
            match passport.get("hgt") {
                None => return false,
                Some(string) => {
                    if !(string.ends_with("cm") || string.ends_with("in")) {
                        return false;
                    };
                    match string.strip_suffix("cm") {
                        None => {}
                        Some(string) => match string.parse::<u32>() {
                            Err(_) => return false,
                            Ok(value) => {
                                if !(value >= 150 && value <= 193) {
                                    return false;
                                }
                            }
                        },
                    };
                    match string.strip_suffix("in") {
                        None => {}
                        Some(string) => match string.parse::<u32>() {
                            Err(_) => return false,
                            Ok(value) => {
                                if !(value >= 59 && value <= 76) {
                                    return false;
                                }
                            }
                        },
                    };
                }
            };
            match passport.get("hcl") {
                None => return false,
                Some(string) => {
                    if !H_COLOR_REGEX.is_match(string) {
                        return false;
                    }
                }
            };
            match passport.get("ecl") {
                None => return false,
                Some(string) => {
                    if !E_COLOR_REGEX.is_match(string) {
                        return false;
                    }
                }
            };
            match passport.get("pid") {
                None => return false,
                Some(string) => {
                    if !PASSPORT_REGEX.is_match(string) {
                        return false;
                    }
                }
            };
            true
        })
        .count();

    println!("{}", valid);
}
