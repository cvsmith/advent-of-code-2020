use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

struct Position {
    state: char,
    change: bool,
    n_occupied_directions: u32,
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
                    n_occupied_directions: 0,
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
                        grid[row][col].n_occupied_directions =
                            _get_n_occupied_directions(&grid, n_rows, n_cols, row, col);
                        if grid[row][col].n_occupied_directions == 0 {
                            seat_changed = true;
                            grid[row][col].change = true;
                        }
                    }
                    OCCUPIED => {
                        occupied_count += 1;
                        grid[row][col].n_occupied_directions =
                            _get_n_occupied_directions(&grid, n_rows, n_cols, row, col);
                        if grid[row][col].n_occupied_directions >= 5 {
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
            break;
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
    println!("empty: {}, occupied: {}", empty_count, occupied_count);
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
    // Need signed for subtraction
    let (signed_row, signed_col) = (row as i32, col as i32);

    // Could generate these with loop...
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut n_occupied_directions = 0;
    for dir in dirs.iter() {
        // Check in that direction until out of bounds or occupied
        let (mut curr_row, mut curr_col) = (signed_row, signed_col);
        loop {
            curr_row = curr_row + dir.0;
            curr_col = curr_col + dir.1;
            // Don't go out of bounds
            if (curr_row < 0)
                || (curr_col < 0)
                || (curr_row as usize >= n_rows)
                || (curr_col as usize >= n_cols)
            {
                break;
            }
            // Stop when occupied
            match grid[curr_row as usize][curr_col as usize].state {
                FLOOR => {}
                EMPTY => {
                    break;
                }
                OCCUPIED => {
                    n_occupied_directions += 1;
                    break;
                }
                _ => println!("invalid position state"),
            }
        }
    }
    return n_occupied_directions;
}

// between 1800 and 3600...
