use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find number of passports, separated by blank lines, with all required fields
fn main() {
    let mut count: i32 = 0;

    let required_fields: HashSet<String> = [
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    ]
    .iter()
    .cloned()
    .collect();
    let optional_fields: HashSet<String> = ["cid".to_string()].iter().cloned().collect();

    let mut curr_fields: HashSet<String> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                // Empty line is a break between passports
                if split_record.len() == 0 {
                    if curr_fields == required_fields {
                        count += 1;
                    }
                    curr_fields.clear();
                    continue;
                }

                for i in 0..split_record.len() {
                    let field_name =
                        split_record[i].split(":").collect::<Vec<&str>>()[0].to_string();
                    if required_fields.contains(&field_name) {
                        curr_fields.insert(field_name);
                    }
                }
            }
        }
    }
    // Lazy fix to off by one error
    if curr_fields == required_fields {
        count += 1;
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
