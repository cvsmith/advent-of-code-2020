use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find number of passwords with between policy_lo and policy_hi occurrences of policy_char
fn main() {
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                let policy_range = split_record[0].split("-").collect::<Vec<&str>>();
                let (policy_lo, policy_hi): (i32, i32) = (
                    policy_range[0].parse().unwrap(),
                    policy_range[1].parse().unwrap(),
                );

                let policy_char = split_record[1].split(":").collect::<Vec<&str>>()[0];

                let password = split_record[2];

                let char_count = password.matches(policy_char).count() as i32;

                if policy_lo <= char_count && char_count <= policy_hi {
                    count += 1;
                }
            }
        }
    }
    println!("count: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
