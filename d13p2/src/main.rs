use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Bus {
    id: u64,
    position: u64,
}

/// Find earliest time where the first bus arrives, followed by all buses in order
/// This is brute-force and works for examples but NOT for actual puzzle input
fn main() {
    let mut arrival_time: u64 = 0;
    let mut buses: Vec<Bus> = Vec::new();
    if let Ok(lines) = _read_lines("./sample.txt") {
        for line in lines {
            if arrival_time == 0 {
                arrival_time = line.unwrap().parse().unwrap();
                continue;
            }
            let full_line = line.unwrap();
            for (pos, id) in full_line.split(',').enumerate() {
                let parsed_id: u64;
                if id == "x" {
                    parsed_id = 1;
                } else {
                    parsed_id = id.parse().unwrap();
                }
                buses.push(Bus { id: parsed_id, position: pos as u64});
            }
        }
    }
    buses.sort_by_key(|bus| bus.id);
    let max_bus = buses.pop().unwrap();
    let mut max_bus_arrival_time: u64 = 0;
    let mut arrival_times: Vec<u64>;
    loop {
        max_bus_arrival_time += max_bus.id;
        arrival_times = [max_bus_arrival_time].to_vec();
        let mut found_conflict = false;
        for other_bus in &buses {
            if other_bus.id == 1 {
                continue;
            }
            if other_bus.position > max_bus.position {
                let other_bus_arrival_time = (1 + (max_bus_arrival_time / other_bus.id)) * other_bus.id;
                if other_bus_arrival_time - max_bus_arrival_time != other_bus.position - max_bus.position{
                    found_conflict = true;
                    break;
                } else {
                    arrival_times.push(other_bus_arrival_time);
                }
            } else {
                let other_bus_arrival_time = (max_bus_arrival_time / other_bus.id) * other_bus.id;
                if max_bus_arrival_time - other_bus_arrival_time != max_bus.position - other_bus.position {
                    found_conflict = true;
                    break;
                } else {
                    arrival_times.push(other_bus_arrival_time);
                }
            }
        }
        if !found_conflict {
            break;
        }
    }
    arrival_times.sort();
    for time in arrival_times {
        println!("time: {}", time);
    }
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
