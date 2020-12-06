use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../input.txt");

fn main() -> io::Result<()> {
    let mut max_seat_id = 0;
    let mut all_seats = vec![false; 1024];

    for line in io::Cursor::new(INPUT).lines() {
        let line = line?;
        let (row_input, col_input) = line.split_at(7);

        let mut row_range: (usize, usize) = (0, 127);
        for c in row_input.chars() {
            match c {
                'B' => row_range = front_half(row_range.0, row_range.1),
                'F' => row_range = back_half(row_range.0, row_range.1),
                _ => panic!("Unrecognized row indicator {}!", c),
            }
        }
        let row = row_range.0;

        let mut col_range: (usize, usize) = (0, 7);
        for c in col_input.chars() {
            match c {
                'L' => col_range = back_half(col_range.0, col_range.1),
                'R' => col_range = front_half(col_range.0, col_range.1),
                _ => panic!("Unrecognized col indicator {}!", c),
            }
        }
        let col = col_range.0;

        let seat_id = (row * 8) + col;
        all_seats[seat_id] = true;
        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("Part One: {}", max_seat_id);

    let mut seat_id = 0;
    for i in 1..1023 {
        if !all_seats[i] && all_seats[i - 1] && all_seats[i + 1] {
            seat_id = i;
            break;
        }
    }
    println!("Part Two: {}", seat_id);
    Ok(())
}

fn front_half(min: usize, max: usize) -> (usize, usize) {
    let range = max - min + 1;
    return (min + range / 2, max);
}

fn back_half(min: usize, max: usize) -> (usize, usize) {
    let range = max - min + 1;
    return (min, max - range / 2);
}
