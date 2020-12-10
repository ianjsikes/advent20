use std::{collections::HashSet, i64};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut valid_count_one = 0;
    let mut valid_count_two = 0;
    for passport in INPUT.split("\n\n") {
        if is_valid_one(passport) {
            valid_count_one += 1;
        }
        if is_valid_two(passport) {
            valid_count_two += 1;
        }
    }
    println!("Part One: {}", valid_count_one);
    println!("Part Two: {}", valid_count_two);
}

fn is_valid_one(pass: &str) -> bool {
    let mut required_props: HashSet<&str> = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*, "cid" */
    ]
    .iter()
    .cloned()
    .collect();

    for line in pass.lines() {
        let pairs = line.split(' ');
        for pair in pairs {
            let key = pair.split(':').next().unwrap();
            required_props.remove(key);
        }
    }
    required_props.is_empty()
}

fn is_valid_two(pass: &str) -> bool {
    let mut required_props: HashSet<&str> = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*, "cid" */
    ]
    .iter()
    .cloned()
    .collect();

    for line in pass.lines() {
        let pairs = line.split(' ');
        for pair in pairs {
            let mut pair = pair.split(':');
            let key = pair.next().unwrap();
            let val = pair.next().unwrap();
            if validate_key(key, val) {
                required_props.remove(key);
            } else {
                return false;
            }
        }
    }
    required_props.is_empty()
}

fn validate_key(key: &str, val: &str) -> bool {
    let valid_ecls: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();

    match key {
        "byr" => (1920..=2002).contains(&val.parse::<usize>().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&val.parse::<usize>().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&val.parse::<usize>().unwrap_or(0)),
        "hgt" => {
            let mut val = val.chars();

            let num_part: usize = (&mut val)
                .take_while(|&c| c != 'c' && c != 'i')
                .collect::<String>()
                .parse()
                .unwrap_or(0);

            // lol idk how 2 rust
            let unit = &(val.collect::<String>())[..];
            match unit {
                "n" => (59..=76).contains(&num_part),
                "m" => (150..=193).contains(&num_part),
                _ => false,
            }
        }
        "hcl" => match val.strip_prefix('#') {
            None => false,
            Some(hex_str) => i64::from_str_radix(hex_str, 16).is_ok(),
        },
        "ecl" => valid_ecls.contains(val),
        "pid" => val.len() == 9 && val.parse::<i32>().unwrap_or(-1) > 0,
        _ => true,
    }
}
