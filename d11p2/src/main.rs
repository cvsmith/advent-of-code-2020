use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

struct Position {
    state: char,
    change: bool,
}

/// Find number of occupied seats after seats stop changing
fn main() {
    // Get grid
    let mut grid: Vec<Vec<Position>> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let mut row: Vec<Position> = Vec::new();
            for char in line.unwrap().chars() {
                row.push(Position {
                    state: char,
                    change: false,
                });
            }
            grid.push(row);
        }
    }

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let (mut empty_count, mut occupied_count) = (0, 0);
    let mut seat_changed = false;
    loop {
        // Figure out which Positions need to change state
        for row in 0..n_rows {
            for col in 0..n_cols {
                match grid[row][col].state {
                    FLOOR => {}
                    EMPTY => {
                        empty_count += 1;
                        if _get_n_occupied_directions(&grid, n_rows, n_cols, row, col) == 0 {
                            seat_changed = true;
                            grid[row][col].change = true;
                        }
                    }
                    OCCUPIED => {
                        occupied_count += 1;
                        if _get_n_occupied_directions(&grid, n_rows, n_cols, row, col) >= 4 {
                            seat_changed = true;
                            grid[row][col].change = true;
                        }
                    }
                    _ => panic!("invalid state: {}", grid[row][col].state),
                }
            }
        }

        // Break when there are no more changes
        if !seat_changed {
            println!("empty: {}, occupied: {}", empty_count, occupied_count);
            return;
        }

        // Execute changes
        for row in 0..n_rows {
            for col in 0..n_cols {
                if grid[row][col].change {
                    match grid[row][col].state {
                        FLOOR => {}
                        EMPTY => {
                            grid[row][col].state = OCCUPIED;
                            grid[row][col].change = false;
                        }
                        OCCUPIED => {
                            grid[row][col].state = EMPTY;
                            grid[row][col].change = false;
                        }
                        _ => panic!("invalid state: {}", grid[row][col].state),
                    }
                }
            }
        }

        // Reset
        seat_changed = false;
        empty_count = 0;
        occupied_count = 0;
    }
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _get_n_occupied_directions(
    grid: &Vec<Vec<Position>>,
    n_rows: usize,
    n_cols: usize,
    row: usize,
    col: usize,
) -> u32 {
    let mut n_adjacent_occupied = 0;

    // Need signed for subtraction
    let (signed_row, signed_col) = (row as i32, col as i32);

    for adj_row in [signed_row - 1, signed_row, signed_row + 1].iter() {
        for adj_col in [signed_col - 1, signed_col, signed_col + 1].iter() {
            // Don't check center of subgrid
            if (*adj_row == signed_row) && (*adj_col == signed_col) {
                continue;
            }
            // Don't go out of bounds
            if (*adj_row < 0)
                || (*adj_col < 0)
                || (*adj_row >= n_rows as i32)
                || (*adj_col >= n_cols as i32)
            {
                continue;
            }
            // Occupied?
            if grid[*adj_row as usize][*adj_col as usize].state == OCCUPIED {
                n_adjacent_occupied += 1;
            }
        }
    }
    return n_adjacent_occupied;
}
