use std::io::{self, BufRead};

const INPUT: &str = include_str!("../input.txt");

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

pub fn part_one() -> io::Result<()> {
    let mut valid_count = 0;
    for line in io::Cursor::new(INPUT).lines() {
        let line = line?;
        let mut chunks = line.split(' ');

        let range_str = chunks.next().unwrap();
        let rule_str = chunks.next().unwrap().trim_end_matches(':');
        let pass_str = chunks.next().unwrap();

        let mut range_split = range_str.split('-');
        let min: usize = range_split.next().unwrap().parse().unwrap();
        let max: usize = range_split.next().unwrap().parse().unwrap();

        let count = pass_str.matches(rule_str).count();
        if count >= min && count <= max {
            valid_count += 1;
        }
    }
    println!("Part One: {}", valid_count);
    Ok(())
}

pub fn part_two() -> io::Result<()> {
    let mut valid_count = 0;
    for line in io::Cursor::new(INPUT).lines() {
        let line = line?;
        let mut chunks = line.split(' ');

        let pos_str = chunks.next().unwrap();
        let rule_char: char = chunks
            .next()
            .unwrap()
            .trim_end_matches(':')
            .parse()
            .unwrap();
        let pass_str = chunks.next().unwrap();

        let mut pos_split = pos_str.split('-');
        let pos1: usize = pos_split.next().unwrap().parse().unwrap();
        let pos2: usize = pos_split.next().unwrap().parse().unwrap();

        let pos1_match = pass_str.chars().nth(pos1 - 1).unwrap() == rule_char;
        let pos2_match = pass_str.chars().nth(pos2 - 1).unwrap() == rule_char;

        // Exclusive OR
        if pos1_match ^ pos2_match {
            valid_count += 1;
        }
    }
    println!("Part Two: {}", valid_count);
    Ok(())
}
