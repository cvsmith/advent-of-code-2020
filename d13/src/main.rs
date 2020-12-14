use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find earliest bus after arrival time, time to wait, and the product of the two
fn main() {
    let mut arrival_time: u64 = 0;
    let mut buses: Vec<u64> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        // if let Ok(lines) = _read_lines("./sample.txt") {
        for line in lines {
            if arrival_time == 0 {
                arrival_time = line.unwrap().parse().unwrap();
                continue;
            }
            let full_line = line.unwrap();
            buses = full_line
                .split(',')
                .into_iter()
                .filter_map(|bus| bus.parse().ok())
                .collect();
        }
    }
    let mut min_remainder = arrival_time;
    let mut min_bus = 0;
    for bus in buses {
        let remainder = ((1 + (arrival_time / bus)) * bus) - arrival_time;
        if remainder < min_remainder {
            min_remainder = remainder;
            min_bus = bus;
        }
    }
    println!(
        "best bus: {}, wait time: {}, product: {}",
        min_bus,
        min_remainder,
        min_bus * min_remainder
    );
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
