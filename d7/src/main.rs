use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MY_COLOR: &str = "shiny gold";
const NO_COLOR: &str = "no other";

/// Find number of colors that can contain a MY_COLOR-colored bag
fn main() {
    // Map colors to the colors they can contain
    let mut outer_to_inner_map: HashMap<String, HashSet<String>> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(rule) = line {
                let split_rule = rule.split("contain").collect::<Vec<&str>>();
                let outer_color = split_rule[0].split("bags").collect::<Vec<&str>>()[0].trim();
                let inner_options = split_rule[1].split(",").collect::<Vec<&str>>();
                let mut inner_colors: HashSet<String> = HashSet::new();
                for option in inner_options {
                    let split_option = option.split_whitespace().collect::<Vec<&str>>();
                    let option_color = format!("{} {}", split_option[1], split_option[2]);
                    if option_color != NO_COLOR {
                        inner_colors.insert(option_color);
                    }
                }
                outer_to_inner_map.insert(outer_color.to_string(), inner_colors);
            }
        }
    }
    assert_eq!(outer_to_inner_map.contains_key(MY_COLOR), true);

    // Invert: map colors to the colors that can contain them
    let mut inner_to_outer_map: HashMap<String, HashSet<String>> = HashMap::new();
    for (outer_color, inner_colors) in outer_to_inner_map.iter() {
        for inner_color in inner_colors {
            if inner_to_outer_map.contains_key(inner_color) {
                let mut existing_outer_colors: HashSet<String> = inner_to_outer_map
                    .get(inner_color)
                    .unwrap()
                    .iter()
                    .cloned()
                    .collect();
                existing_outer_colors.insert(outer_color.to_string());
                inner_to_outer_map.insert(inner_color.to_string(), existing_outer_colors);
            } else {
                let mut new_outer_colors: HashSet<String> = HashSet::new();
                new_outer_colors.insert(outer_color.to_string());
                inner_to_outer_map.insert(inner_color.to_string(), new_outer_colors);
            }
        }
    }
    assert_eq!(inner_to_outer_map.contains_key(MY_COLOR), true);

    // Get colors that can contain MY_COLOR
    let mut visited_set: HashSet<String> = inner_to_outer_map
        .get(MY_COLOR)
        .unwrap()
        .iter()
        .cloned()
        .collect();
    let mut new_set: HashSet<String> = visited_set.iter().cloned().collect();
    loop {
        let mut curr_set: HashSet<String> = HashSet::new();
        for curr_color in new_set.drain() {
            if inner_to_outer_map.contains_key(&curr_color) {
                for new_color in inner_to_outer_map.get(&curr_color).unwrap().iter() {
                    if !visited_set.contains(new_color) {
                        curr_set.insert(new_color.to_string());
                    }
                }
            }
            visited_set.insert(curr_color);
        }
        if curr_set.is_empty() {
            break;
        } else {
            new_set = curr_set;
        }
    }

    println!("num colors: {}", visited_set.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
