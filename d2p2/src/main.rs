use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find number of passwords where exactly one of policy_lo or policy_hi contains policy_char
fn main() {
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                let policy_range = split_record[0].split("-").collect::<Vec<&str>>();
                let (policy_lo, policy_hi): (usize, usize) = (
                    policy_range[0].parse().unwrap(),
                    policy_range[1].parse().unwrap(),
                );

                let policy_char = split_record[1].split(":").collect::<Vec<&str>>()[0];

                let password = split_record[2];

                // lo and hi are not zero-indexed (`1` is 1st character in password)
                let (password_char_lo, password_char_hi) = (
                    &password[policy_lo - 1..policy_lo],
                    &password[policy_hi - 1..policy_hi],
                );

                if (password_char_lo == policy_char) ^ (password_char_hi == policy_char) {
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
