use array_tool::vec::Shift;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let nums: Vec<i64> = INPUT
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut weakness_val: i64 = 0;
    let mut weakness_idx: usize = 0;

    for (i, &num) in nums.iter().enumerate().skip(25) {
        let prev_nums = &nums[(i - 25)..i];
        let valid = two_sum(prev_nums, num);
        if !valid {
            weakness_val = num;
            weakness_idx = i;
            println!("Part One: {}", num);
            break;
        }
    }

    let mut window: Vec<i64> = vec![];
    for &num in nums.iter().take(weakness_idx).rev() {
        window.push(num);

        let mut sum: i64 = window.iter().sum();
        if sum > weakness_val {
            window.shift();
            sum = window.iter().sum();
        }
        if sum == weakness_val {
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            println!("Part Two: {}", min + max);
            break;
        }
    }
}

pub fn two_sum(nums: &[i64], target: i64) -> bool {
    let mut diff_set: HashSet<i64> = HashSet::with_capacity(nums.len());

    for &number in nums {
        let diff = target - number;
        if diff_set.contains(&diff) {
            return true;
        }
        diff_set.insert(number);
    }

    false
}
