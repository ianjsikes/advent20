use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let target: i32 = 2020;

    for (idx, num1) in numbers.iter().enumerate() {
        let inner_target = target - num1;
        let mut diff_set: HashSet<i32> = HashSet::new();

        for x in (idx + 1)..(numbers.len() - 1) {
            let num2 = numbers[x];
            let diff = inner_target - num2;
            if diff_set.contains(&diff) {
                println!("Solution {}", diff * num2 * num1);
                return;
            }
            diff_set.insert(num2);
        }
    }
}
