use array_tool::vec::Shift;
use std::collections::HashMap;
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut jolt_vals: Vec<i32> = INPUT
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    jolt_vals.sort_unstable();
    jolt_vals.push(jolt_vals.last().unwrap() + 3);
    jolt_vals.unshift(0);

    part_one(&jolt_vals);
    part_two(&jolt_vals);
}

pub fn part_one(jolt_vals: &[i32]) {
    let mut jolt_diffs: HashMap<i32, i32> = HashMap::new();
    let mut curr_joltage = 0;

    for val in jolt_vals {
        let diff = *val - curr_joltage;
        *jolt_diffs.entry(diff).or_insert(0) += 1;
        curr_joltage = *val;
    }

    let num_ones = *jolt_diffs.get(&1).unwrap_or(&0);
    let num_threes = *jolt_diffs.get(&3).unwrap_or(&0);
    println!("Part One: {}", num_ones * num_threes);
}

pub fn part_two(jolt_vals: &[i32]) {
    // Stand back, I am about to attempt _DYNAMIC PROGRAMMING_
    let len = jolt_vals.len();
    let mut combos: Vec<i128> = vec![0; len];
    combos[len - 1] = 1;
    for (i, &val) in jolt_vals.iter().enumerate().rev() {
        let mut j = i + 1;
        while j < len && jolt_vals[j] - val <= 3 {
            combos[i] += combos[j];
            j += 1;
        }
    }

    println!("Part Two: {}", combos[0]);
}
