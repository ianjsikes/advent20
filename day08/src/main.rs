#![feature(str_split_once)]

use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let ops: Vec<OpCode> = INPUT
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("nop", _) => OpCode::Nop,
            ("acc", n) => OpCode::Acc(n.parse().unwrap()),
            ("jmp", n) => OpCode::Jmp(n.parse().unwrap()),
            _ => panic!("Invalid op code {}", line),
        })
        .collect();

    let mut visited: HashSet<usize> = HashSet::with_capacity(ops.len());
    // The value of the global register
    let mut acc: isize = 0;
    // The index of the operation being executed
    let mut cursor: usize = 0;

    while visited.insert(cursor) {
        match ops[cursor] {
            OpCode::Nop => cursor += 1,
            OpCode::Acc(n) => {
                acc += n;
                cursor += 1;
            }
            OpCode::Jmp(n) => cursor = cursor.wrapping_add(n as usize),
        }
    }

    println!("Part One: {}", acc);
}

#[derive(Debug)]
pub enum OpCode {
    Nop,
    Acc(isize),
    Jmp(isize),
}
