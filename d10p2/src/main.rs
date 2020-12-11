use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_JOLT_DIFF: u32 = 3;

struct AdapterPath {
    num: u32,
    n_paths: u64,
}

/// Find total number of possible paths from first to last adapter
fn main() {
    // Get sorted nums: (num, n_paths)
    let mut nums: Vec<AdapterPath> = Vec::new();
    nums.push(AdapterPath { num: 0, n_paths: 0 });
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let line = line.unwrap();
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            nums.push(AdapterPath {
                num: split_line[0].parse().unwrap(),
                n_paths: 0,
            });
        }
    }
    nums.sort_by_key(|num| num.num);

    // Bootstrap - set 1 path from last adapter to target
    nums.last_mut().unwrap().n_paths = 1;

    // Work backwards through adapters
    for i in (0..nums.len()).rev() {
        // Num paths from each adapter is sum of num paths from adapters it can reach
        for lookahead in 1..(MAX_JOLT_DIFF as usize) + 1 {
            if lookahead + i < nums.len() {
                if nums[lookahead + i].num <= nums[i].num + MAX_JOLT_DIFF {
                    nums[i].n_paths += nums[lookahead + i].n_paths;
                }
            }
        }
    }

    println!("n_paths from first to last: {}", nums[0].n_paths);
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
