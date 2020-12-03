use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TREE_CHAR: &str = "#";
const N_MOVES_RIGHT: u32 = 3;

/// Find number of trees encountered from (0,0) moving down 1, right N_MOVES_RIGHT
fn main() {
    let (mut row_num, mut tree_count): (u32, u32) = (0, 0);
    let mut row_length: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                if row_num == 0 {
                    row_length = row.len() as u32;
                }

                let index: usize = ((row_num * N_MOVES_RIGHT) % row_length) as usize;
                if &row[index..index + 1] == TREE_CHAR {
                    tree_count += 1;
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
