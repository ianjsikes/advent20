use std::{collections::HashMap, io};

const INPUT: &str = include_str!("../input.txt");

fn main() -> io::Result<()> {
    let mut first_sum = 0;
    let mut second_sum = 0;
    for group in INPUT.split("\n\n") {
        let mut answers: HashMap<char, usize> = HashMap::new();
        let lines = group.lines();
        let num_people = lines.clone().count();
        for line in lines {
            for c in line.chars() {
                *answers.entry(c).or_insert(0) += 1;
            }
        }
        let count = answers.iter().fold(
            0,
            |sum, (_, &num)| if num == num_people { sum + 1 } else { sum },
        );
        first_sum += answers.len();
        second_sum += count;
    }
    println!("Part One: {}", first_sum);
    println!("Part Two: {}", second_sum);
    Ok(())
}
