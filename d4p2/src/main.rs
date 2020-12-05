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
    let _optional_fields: HashSet<String> = ["cid".to_string()].iter().cloned().collect();

    let mut valid_fields: HashSet<String> = HashSet::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(record) = line {
                let split_record = record.split_whitespace().collect::<Vec<&str>>();

                // Empty line is a break between passports
                if split_record.len() == 0 {
                    if valid_fields == required_fields {
                        count += 1;
                    }
                    valid_fields.clear();
                    continue;
                }

                for i in 0..split_record.len() {
                    let field = split_record[i].split(":").collect::<Vec<&str>>();
                    let (field_name, field_val) = (field[0].to_string(), field[1].to_string());

                    if required_fields.contains(&field_name) {
                        match field_name.as_str() {
                            "byr" => {
                                if !is_valid_year(field_val, 1920, 2002) {
                                    continue;
                                }
                            }
                            "iyr" => {
                                if !is_valid_year(field_val, 2010, 2020) {
                                    continue;
                                }
                            }
                            "eyr" => {
                                if !is_valid_year(field_val, 2020, 2030) {
                                    continue;
                                }
                            }
                            "hgt" => {
                                if !is_valid_height(field_val) {
                                    continue;
                                }
                            }
                            "hcl" => {
                                if !is_valid_hcl(field_val) {
                                    continue;
                                }
                            }
                            "ecl" => {
                                if !is_valid_ecl(field_val) {
                                    continue;
                                }
                            }
                            "pid" => {
                                if !is_valid_pid(field_val) {
                                    continue;
                                }
                            }
                            &_ => {
                                continue;
                            }
                        }
                        // passed checks
                        valid_fields.insert(field_name);
                    }
                }
            }
        }
    }
    // Lazy fix to off by one error
    if valid_fields == required_fields {
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

fn is_valid_year(byr: String, min: u32, max: u32) -> bool {
    if byr.len() != 4 {
        return false;
    }
    let byr = byr.parse().unwrap();
    return is_in_range(byr, min, max);
}

fn is_valid_height(hgt: String) -> bool {
    let cm_idx = hgt.find("cm");
    if cm_idx.is_some() {
        let cm_val = hgt[0..cm_idx.unwrap()].parse::<u32>().unwrap();
        return is_in_range(cm_val, 150, 193);
    }
    let in_idx = hgt.find("in");
    if in_idx.is_some() {
        let in_val = hgt[0..in_idx.unwrap()].parse::<u32>().unwrap();
        return is_in_range(in_val, 59, 76);
    }
    return false;
}

// Inclusive
fn is_in_range(val: u32, min: u32, max: u32) -> bool {
    if !(min <= val && val <= max) {
        return false;
    }
    return true;
}

fn is_valid_hcl(hcl: String) -> bool {
    if hcl[0..1] != "#".to_string() {
        return false;
    }
    let color = hcl[1..].to_string();
    if color.len() != 6 {
        return false;
    }
    for c in color.chars() {
        if c.is_ascii_digit() {
            return true;
        }
        if c.is_ascii_lowercase() && c <= 'f' {
            return true;
        }
    }
    return false;
}

fn is_valid_ecl(ecl: String) -> bool {
    match ecl.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        &_ => return false,
    }
}

fn is_valid_pid(pid: String) -> bool {
    if pid.len() != 9 {
        return false;
    }
    for c in pid.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    return true;
}
