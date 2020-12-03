use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TREE_CHAR: &str = "#";
const N_MOVES_RIGHT: u32 = 1;
const N_MOVES_DOWN: u32 = 2;

/// Find number of trees encountered from (0,0) moving right N_MOVES_RIGHT and down N_MOVES_DOWN
/// At least works for N_MOVES_DOWN={1,2}
fn main() {
    let (mut row_num, mut tree_count): (u32, u32) = (0, 0);
    let mut row_length: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                if row_num == 0 {
                    row_length = row.len() as u32;
                }
                // Only move right and check for trees on every
                if row_num % N_MOVES_DOWN == 0 {
                    let row_index: u32 = row_num / N_MOVES_DOWN;

                    let index: usize = ((row_index * N_MOVES_RIGHT) % row_length) as usize;
                    if &row[index..index + 1] == TREE_CHAR {
                        tree_count += 1;
                    }
                }
                row_num += 1;
            }
        }
    }
    println!("count: {}", tree_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
