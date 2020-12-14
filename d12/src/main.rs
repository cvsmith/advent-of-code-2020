use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Direction {
    row_diff: i32,
    col_diff: i32,
}

struct Ship {
    row: i32,
    col: i32,
    heading: char,
}

const NORTH: char = 'N';
const SOUTH: char = 'S';
const EAST: char = 'E';
const WEST: char = 'W';
const LEFT: char = 'L';
const RIGHT: char = 'R';
const FORWARD: char = 'F';

/// Find Manhattan distance from starting point after sequence of navigation instructions
fn main() {
    // Build direction map
    let mut directions: HashMap<char, Direction> = HashMap::new();
    directions.insert(
        NORTH,
        Direction {
            row_diff: -1,
            col_diff: 0,
        },
    );
    directions.insert(
        SOUTH,
        Direction {
            row_diff: 1,
            col_diff: 0,
        },
    );
    directions.insert(
        EAST,
        Direction {
            row_diff: 0,
            col_diff: 1,
        },
    );
    directions.insert(
        WEST,
        Direction {
            row_diff: 0,
            col_diff: -1,
        },
    );

    // Build turn map
    let mut turns: HashMap<char, HashMap<char, char>> = HashMap::new();
    let mut left_map: HashMap<char, char> = HashMap::new();
    left_map.insert(NORTH, WEST);
    left_map.insert(SOUTH, EAST);
    left_map.insert(EAST, NORTH);
    left_map.insert(WEST, SOUTH);
    turns.insert(LEFT, left_map);

    let mut right_map: HashMap<char, char> = HashMap::new();
    right_map.insert(NORTH, EAST);
    right_map.insert(SOUTH, WEST);
    right_map.insert(EAST, SOUTH);
    right_map.insert(WEST, NORTH);
    turns.insert(RIGHT, right_map);

    // Initialize ship
    let mut ship = Ship {
        row: 0,
        col: 0,
        heading: EAST,
    };

    // Loop through instructions and change ship state
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let instruction = line.unwrap();
            let (action, value) = instruction.split_at(1);
            // Parse
            let action: char = action.chars().collect::<Vec<char>>()[0];
            let value: i32 = value.parse().unwrap();
            if action == FORWARD {
                let dir = directions.get(&ship.heading).unwrap();
                ship.row += value * dir.row_diff;
                ship.col += value * dir.col_diff;
            } else if directions.contains_key(&action) {
                let dir = directions.get(&action).unwrap();
                ship.row += value * dir.row_diff;
                ship.col += value * dir.col_diff;
            } else if turns.contains_key(&action) {
                let turn = turns.get(&action).unwrap();
                let mut dir = &ship.heading;
                let n_turns = value / 90;
                for _ in 0..n_turns {
                    dir = turn.get(dir).unwrap();
                }
                ship.heading = *dir;
            }
        }
    }
    println!(
        "row: {}, col: {}, distance: {}",
        ship.row,
        ship.col,
        ship.row.abs() + ship.col.abs()
    );
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
