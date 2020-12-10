use std::collections::HashSet;
use std::fs;

fn main() {
    part_one();
    part_two();
}

pub fn part_one() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let mut diff_set: HashSet<i32> = HashSet::with_capacity(numbers.len());

    let target: i32 = 2020;
    for number in numbers {
        let diff = target - number;
        if diff_set.contains(&diff) {
            println!("Part One Solution: {}", diff * number);
            return;
        }
        diff_set.insert(number);
    }
}

pub fn part_two() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let target: i32 = 2020;

    for (idx, num1) in numbers.iter().enumerate() {
        let inner_target = target - num1;
        let mut diff_set: HashSet<i32> = HashSet::new();

        for &num2 in numbers.iter().skip(idx) {
            let diff = inner_target - num2;
            if diff_set.contains(&diff) {
                println!("Part Two Solution {}", diff * num2 * num1);
                return;
            }
            diff_set.insert(num2);
        }
    }
}
