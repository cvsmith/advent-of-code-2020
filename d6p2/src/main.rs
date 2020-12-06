use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find number of "yes" responses to unique questions from each group
fn main() {
    let mut count: usize = 0;

    let mut group_responses: HashSet<char> = HashSet::new();
    let mut new_group: bool = true;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                // Empty line is a break between groups
                if split_record.len() == 0 {
                    count += group_responses.len();
                    group_responses.clear();
                    new_group = true;
                    continue;
                }

                let person_responses: HashSet<char> = split_record[0].chars().collect();
                if new_group {
                    group_responses = person_responses.iter().cloned().collect();
                    new_group = false;
                } else {
                    group_responses = group_responses
                        .intersection(&person_responses)
                        .cloned()
                        .collect();
                }
            }
        }
    }
    // Lazy fix to off-by-one error
    count += group_responses.len();

    println!("count: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
