use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_JOLT_DIFF: u32 = 3;

/// Find product of # of 1-jolt diffs and # of 3-volt diffs in a chain that uses all adapters
fn main() {
    // Get sorted nums
    let mut nums: Vec<u32> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let line = line.unwrap();
            let split_line = line.split_whitespace().collect::<Vec<&str>>();
            nums.push(split_line[0].parse().unwrap());
        }
    }
    nums.sort();

    // Count diffs
    let mut jolt_diffs: HashMap<u32, u32> = HashMap::new();
    for diff in 1..MAX_JOLT_DIFF + 1 {
        jolt_diffs.insert(diff, 0);
    }
    for i in 0..nums.len() {
        let diff: u32;
        if i == 0 {
            diff = nums[i];
        } else {
            diff = nums[i] - nums[i - 1];
        }
        jolt_diffs.entry(diff).and_modify(|count| *count += 1);
    }

    // Add last diff (device has a jolt rating 3 higher than max adapter)
    jolt_diffs
        .entry(3)
        .and_modify(|count| *count += 1)
        .or_insert(0);

    // Print result
    let (diff_1, diff_3) = (jolt_diffs.get(&1).unwrap(), jolt_diffs.get(&3).unwrap());
    println!(
        "num 1-jolt diffs: {}, num 3-jolt diffs: {}, product: {}",
        diff_1,
        diff_3,
        diff_1 * diff_3
    );
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
