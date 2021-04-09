use array2d::Array2D;
use std::{thread, time};
use termion::{clear, color};

const INPUT: &str = include_str!("../input.txt");
// const INPUT: &str = "...\n.L.\n...";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Floor,
    Empty,
    Filled,
}

fn main() {
    let parsed_input: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Floor,
                    '#' => Cell::Filled,
                    'L' => Cell::Empty,
                    _ => panic!("Unrecognized input char {}", c),
                })
                .collect()
        })
        .collect();
    let mut grid = Array2D::from_rows(&parsed_input);

    // print_grid(&grid);
    // thread::sleep(time::Duration::from_millis(200));
    // println!("{}", clear::All);

    loop {
        let new_grid = step_two(&grid);

        // print_grid(&new_grid);
        // thread::sleep(time::Duration::from_millis(200));
        // println!("{}", clear::All);

        if grid == new_grid {
            break;
        }
        grid = new_grid;
    }

    println!("Part Two: {}", num_occupied(&grid));
}

pub fn print_grid(grid: &Array2D<Cell>) {
    let mut grid_str = "".to_string();
    for row_iter in grid.rows_iter() {
        for &cell in row_iter {
            match cell {
                Cell::Floor => {
                    grid_str
                        .push_str(format!("{white}.", white = color::Fg(color::White)).as_str());
                }
                Cell::Filled => {
                    grid_str.push_str(format!("{red}#", red = color::Fg(color::Red)).as_str());
                }
                Cell::Empty => {
                    grid_str
                        .push_str(format!("{green}L", green = color::Fg(color::Green)).as_str());
                }
            }
        }
        grid_str.push('\n');
    }
    print!("{}", grid_str);
}

pub fn step_one(grid: &Array2D<Cell>) -> Array2D<Cell> {
    let mut new_grid = grid.clone();
    for (y, row_iter) in grid.rows_iter().enumerate() {
        for (x, _cell) in row_iter.enumerate() {
            let new_cell = step_cell_one(&grid, x, y);
            new_grid.set(y, x, new_cell).unwrap();
        }
    }

    new_grid
}

pub fn step_two(grid: &Array2D<Cell>) -> Array2D<Cell> {
    let mut new_grid = grid.clone();
    for (y, row_iter) in grid.rows_iter().enumerate() {
        for (x, _cell) in row_iter.enumerate() {
            let new_cell = step_cell_two(&grid, x, y);
            new_grid.set(y, x, new_cell).unwrap();
        }
    }

    new_grid
}

pub fn step_cell_one(grid: &Array2D<Cell>, x: usize, y: usize) -> Cell {
    match &grid.get(y, x).unwrap() {
        Cell::Floor => Cell::Floor,
        Cell::Empty => {
            if occupied_neighbors(grid, x, y) == 0 {
                Cell::Filled
            } else {
                Cell::Empty
            }
        }
        Cell::Filled => {
            if occupied_neighbors(grid, x, y) >= 4 {
                Cell::Empty
            } else {
                Cell::Filled
            }
        }
    }
}

pub fn step_cell_two(grid: &Array2D<Cell>, x: usize, y: usize) -> Cell {
    match &grid.get(y, x).unwrap() {
        Cell::Floor => Cell::Floor,
        Cell::Empty => {
            if occupied_neighbors_two(grid, x, y) == 0 {
                Cell::Filled
            } else {
                Cell::Empty
            }
        }
        Cell::Filled => {
            if occupied_neighbors_two(grid, x, y) >= 5 {
                Cell::Empty
            } else {
                Cell::Filled
            }
        }
    }
}

pub fn occupied_neighbors(grid: &Array2D<Cell>, x: usize, y: usize) -> u8 {
    let mut neighbor_coords: Vec<(usize, usize)> = vec![(y, x + 1), (y + 1, x), (y + 1, x + 1)];
    // I know there is a better way to do this but I don't wanna deal with unsigned integers right now
    if x > 0 && y > 0 {
        neighbor_coords.append(&mut vec![
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x - 1),
            (y + 1, x - 1),
        ]);
    } else if x > 0 {
        neighbor_coords.append(&mut vec![(y, x - 1), (y + 1, x - 1)]);
    } else if y > 0 {
        neighbor_coords.append(&mut vec![(y - 1, x), (y - 1, x + 1)]);
    }
    return neighbor_coords.iter().fold(0, |count, (row, col)| {
        let neighbor = grid.get(*row, *col);
        match neighbor {
            Some(&Cell::Filled) => count + 1,
            _ => count,
        }
    });
}

pub fn occupied_neighbors_two(grid: &Array2D<Cell>, x: usize, y: usize) -> u8 {
    let dirs: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    return dirs.iter().fold(0, |count, &dir| {
        if check_direction(grid, (y, x), dir) {
            count + 1
        } else {
            count
        }
    });
}

pub fn check_direction(
    grid: &Array2D<Cell>,
    (row, col): (usize, usize),
    (d_row, d_col): (isize, isize),
) -> bool {
    let width = grid.row_len();
    let height = grid.column_len();

    let mut curr_row = row;
    let mut curr_col = col;

    loop {
        curr_row = curr_row.wrapping_add(d_row as usize);
        curr_col = curr_col.wrapping_add(d_col as usize);

        if curr_row >= height || curr_col >= width {
            return false;
        }

        let &cell = grid.get(curr_row, curr_col).unwrap();
        if cell == Cell::Filled {
            return true;
        }
        if cell == Cell::Empty {
            return false;
        }
    }
}

pub fn num_occupied(grid: &Array2D<Cell>) -> u32 {
    let mut num: u32 = 0;
    for row_iter in grid.rows_iter() {
        for &cell in row_iter {
            if cell == Cell::Filled {
                num += 1;
            }
        }
    }
    num
}
