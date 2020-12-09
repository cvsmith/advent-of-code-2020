use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TARGET: u64 = 2089807806;

/// Find contiguous segment that sums to TARGET, then get sum of min and max in that segment
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
    for lo in 0..nums.len() {
        for hi in lo + 1..nums.len() {
            let slice = &nums[lo..hi];
            let sum: u64 = slice.iter().sum();
            if sum == TARGET {
                println!(
                    "found slice that sums to {}, lo: {}, hi: {}",
                    TARGET, lo, hi
                );
                let mut sorted = slice.iter().cloned().collect::<Vec<u64>>();
                sorted.sort();
                let (min, max) = (sorted[0], sorted.last().unwrap());
                println!("min: {}, max: {}", min, max);
                println!("min + max: {}", min + max);
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
