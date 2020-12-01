use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TARGET_SUM: i32 = 2020;

/// Find trio of entries that sum to TARGET_SUM and compute their product
fn main() {
    let mut entries = Vec::<i32>::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(num) = line {
                entries.push(num.parse().unwrap());
            }
        }
    }

    for i in 0..entries.len() {
        for j in i..entries.len() {
            for k in j..entries.len() {
                let (entry_i, entry_j, entry_k) = (entries[i], entries[j], entries[k]);
                if entry_i + entry_j + entry_k == TARGET_SUM {
                    println!("entries: {}, {}, {}", entry_i, entry_j, entry_k);
                    println!("product: {}", entry_i * entry_j * entry_k);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
