use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MY_COLOR: &str = "shiny gold";
const NO_COLOR: &str = "other bags.";

/// Find number of bags that must be contained in a MY_COLOR-colored bag
fn main() {
    // Map colors to the colors they must contain and their amounts
    let mut outer_to_inner_map: HashMap<String, HashSet<(String, u32)>> = HashMap::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            if let Ok(rule) = line {
                let split_rule = rule.split("contain").collect::<Vec<&str>>();
                let outer_color = split_rule[0].split("bags").collect::<Vec<&str>>()[0].trim();
                let inner_options = split_rule[1].split(",").collect::<Vec<&str>>();
                let mut inner_colors: HashSet<(String, u32)> = HashSet::new();
                for option in inner_options {
                    let split_option = option.split_whitespace().collect::<Vec<&str>>();
                    let mut option_amount: u32 = 0;
                    if split_option[0] != "no" {
                        option_amount = split_option[0].parse().unwrap();
                    }
                    let option_color = format!("{} {}", split_option[1], split_option[2]);
                    if option_color != NO_COLOR {
                        inner_colors.insert((option_color, option_amount));
                    }
                }
                outer_to_inner_map.insert(outer_color.to_string(), inner_colors);
            }
        }
    }
    assert_eq!(outer_to_inner_map.contains_key(MY_COLOR), true);
    println!(
        "num bags: {}",
        _get_num_bags_in_bag(&outer_to_inner_map, MY_COLOR.to_string())
    );
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _get_num_bags_in_bag(
    outer_to_inner_map: &HashMap<String, HashSet<(String, u32)>>,
    color: String,
) -> u32 {
    let mut bags_in_bag = 0;
    if outer_to_inner_map.contains_key(&color) {
        for color_amount in outer_to_inner_map.get(&color).unwrap() {
            let (color, amount) = color_amount;
            bags_in_bag +=
                amount + amount * _get_num_bags_in_bag(outer_to_inner_map, color.to_string());
        }
    }
    return bags_in_bag;
}
