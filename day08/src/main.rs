#![feature(str_split_once)]

use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let ops: Vec<OpCode> = INPUT
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("nop", n) => OpCode::Nop(n.parse().unwrap()),
            ("acc", n) => OpCode::Acc(n.parse().unwrap()),
            ("jmp", n) => OpCode::Jmp(n.parse().unwrap()),
            _ => panic!("Invalid op code {}", line),
        })
        .collect();

    let part_one_acc = execute_prog(&ops).0;
    println!("Part One: {}", part_one_acc);

    for (i, op) in ops.iter().enumerate() {
        let new_op = match *op {
            OpCode::Acc(_) => continue,
            OpCode::Nop(n) => OpCode::Jmp(n),
            OpCode::Jmp(n) => OpCode::Nop(n),
        };

        let mut new_ops = ops.clone();
        new_ops[i] = new_op;

        let (acc, success) = execute_prog(&new_ops);
        if success {
            println!("Part Two: {}", acc);
            break;
        }
    }
}

pub fn execute_prog(ops: &[OpCode]) -> (isize, bool) {
    let mut visited: HashSet<usize> = HashSet::with_capacity(ops.len());
    // The value of the global register
    let mut acc: isize = 0;
    // The index of the operation being executed
    let mut cursor: usize = 0;

    while visited.insert(cursor) && cursor < ops.len() {
        match ops[cursor] {
            OpCode::Nop(_) => cursor += 1,
            OpCode::Acc(n) => {
                acc += n;
                cursor += 1;
            }
            OpCode::Jmp(n) => cursor = cursor.wrapping_add(n as usize),
        }
    }

    (acc, cursor == ops.len())
}

#[derive(Debug, Clone)]
pub enum OpCode {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}
