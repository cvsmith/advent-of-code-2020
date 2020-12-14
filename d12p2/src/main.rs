use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{collections::HashMap, io::Error};

struct Direction {
    row_diff: i32,
    col_diff: i32,
}

struct Ship {
    row: i32,
    col: i32,
    waypoint_row_diff: i32,
    waypoint_col_diff: i32,
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
    let directions: HashMap<char, Direction> = _get_directions();

    // Initialize ship
    let mut ship = _get_ship();

    // Loop through instructions and change ship state
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let (action, value) = _get_action_and_value(line);
            if action == FORWARD {
                ship.row += value * ship.waypoint_row_diff;
                ship.col += value * ship.waypoint_col_diff;
            } else if directions.contains_key(&action) {
                let dir = directions.get(&action).unwrap();
                ship.waypoint_row_diff += value * dir.row_diff;
                ship.waypoint_col_diff += value * dir.col_diff;
            } else {
                let n_turns = value / 90;
                for _ in 0..n_turns {
                    let (row, col) = (ship.waypoint_row_diff, ship.waypoint_col_diff);
                    if action == LEFT {
                        ship.waypoint_row_diff = col * -1;
                        ship.waypoint_col_diff = row;
                    } else if action == RIGHT {
                        ship.waypoint_row_diff = col;
                        ship.waypoint_col_diff = row * -1;
                    } else {
                        println!("invalid action: {}", action);
                    }
                }
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

fn _get_directions() -> HashMap<char, Direction> {
    let mut directions = HashMap::new();
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
    return directions;
}

fn _get_ship() -> Ship {
    return Ship {
        row: 0,
        col: 0,
        waypoint_row_diff: -1,
        waypoint_col_diff: 10,
    };
}

fn _get_action_and_value(line: Result<String, Error>) -> (char, i32) {
    let instruction = line.unwrap();
    let (action, value) = instruction.split_at(1);
    // Parse
    let action: char = action.chars().collect::<Vec<char>>()[0];
    let value: i32 = value.parse().unwrap();
    return (action, value);
}
