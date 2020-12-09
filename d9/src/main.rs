use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const LOOKBACK_LENGTH: usize = 25;

/// Find first number that isn't the sum of two numbers in the previous 25 numbers
fn main() {
    // Get nums
    let mut nums: Vec<u64> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let line = line.unwrap();
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            nums.push(split_line[0].parse().unwrap());
        }
    }
    for (curr_i, curr_num) in nums.iter().enumerate() {
        if curr_i >= LOOKBACK_LENGTH {
            let mut found_sum = false;
            for pair in nums[curr_i - LOOKBACK_LENGTH..curr_i]
                .into_iter()
                .combinations(2)
            {
                if pair[0] + pair[1] == *curr_num {
                    found_sum = true;
                    break;
                }
            }
            if !found_sum {
                println!("no sum found, line: {}, num: {}", curr_i, curr_num);
                return;
            }
        }
    }
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
