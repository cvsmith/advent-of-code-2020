use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find number of "yes" responses to unique questions from each group
fn main() {
    let mut count: usize = 0;

    let mut curr_responses: HashSet<char> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                // Empty line is a break between groups
                if split_record.len() == 0 {
                    count += curr_responses.len();
                    curr_responses.clear();
                    continue;
                }

                let responses = split_record[0];
                for c in responses.chars() {
                    curr_responses.insert(c);
                }
            }
        }
    }
    // Lazy fix to off-by-one error
    count += curr_responses.len();

    println!("count: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
