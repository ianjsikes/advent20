use std::collections::HashSet;
use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let mut diff_set: HashSet<i32> = HashSet::with_capacity(numbers.len());

    let target: i32 = 2020;
    for number in numbers {
        let diff = target - number;
        if diff_set.contains(&diff) {
            println!("Solution: {}", diff * number);
            return;
        }
        diff_set.insert(number);
    }
}
