use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> io::Result<()> {
    let mut product = 1;
    product *= num_trees(INPUT, 1, 1)?;
    product *= num_trees(INPUT, 1, 3)?;
    product *= num_trees(INPUT, 1, 5)?;
    product *= num_trees(INPUT, 1, 7)?;
    product *= num_trees(INPUT, 2, 1)?;
    println!("Hit {} trees!", product);
    Ok(())
}

fn num_trees(input: &str, down_step: usize, right_step: usize) -> io::Result<usize> {
    let mut tree_count = 0;
    let mut x_pos = 0;
    for line in io::Cursor::new(input)
        .lines()
        .skip(down_step)
        .step_by(down_step)
    {
        let line = line?;

        x_pos += right_step;
        x_pos = x_pos % line.len();
        let char_at_pos = line.chars().skip(x_pos).next().unwrap();
        if char_at_pos == '#' {
            tree_count += 1;
        }
    }
    Ok(tree_count)
}
