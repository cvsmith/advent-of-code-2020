use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct InstructionSet {
    acc: String,
    jmp: String,
    nop: String,
}
struct Instruction {
    instruction: String,
    value: i32,
}

/// Fix the program by making one jmp <-> nop change, then find acc value at termination
fn main() {
    let instruction_set: InstructionSet = InstructionSet {
        acc: "acc".to_string(),
        jmp: "jmp".to_string(),
        nop: "nop".to_string(),
    };

    // (instruction, visited)
    let mut instructions: Vec<Instruction> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let curr_instruction = line.unwrap();
            let split_instruction = curr_instruction.split_whitespace().collect::<Vec<&str>>();
            let instruction = split_instruction[0];
            let value: i32 = split_instruction[1].parse().unwrap();
            let instruction: Instruction = Instruction {
                instruction: instruction.to_string(),
                value: value,
            };
            instructions.push(instruction);
        }
    }

    for change_ptr in 0..instructions.len() {
        // Change one nop <-> jmp at a time
        if instructions[change_ptr].instruction == instruction_set.nop {
            instructions[change_ptr].instruction = instruction_set.jmp.to_string();
        } else if instructions[change_ptr].instruction == instruction_set.jmp {
            instructions[change_ptr].instruction = instruction_set.nop.to_string();
        }

        let mut acc: i32 = 0;
        let mut ptr: usize = 0;
        let mut visited_set: HashSet<usize> = HashSet::new();

        // Run program
        loop {
            // Check termination
            if ptr == instructions.len() {
                println!("program finished. acc: {}", acc);
                return;
            }

            // Check whether already visited this instruction
            if visited_set.contains(&ptr) {
                break;
            }
            visited_set.insert(ptr);

            // Execute instruction
            let curr_instruction = &instructions[ptr];
            let new_ptr: usize;
            if curr_instruction.instruction == instruction_set.acc {
                acc += curr_instruction.value;
                new_ptr = ptr + 1;
            } else if curr_instruction.instruction == instruction_set.jmp {
                new_ptr = ((ptr as i32) + curr_instruction.value) as usize;
            } else if curr_instruction.instruction == instruction_set.nop {
                new_ptr = ptr + 1;
            } else {
                panic!("invalid instruction: {}", curr_instruction.instruction);
            }

            ptr = new_ptr;
        }

        // Undo nop <-> jmp change
        if instructions[change_ptr].instruction == instruction_set.nop {
            instructions[change_ptr].instruction = instruction_set.jmp.to_string();
        } else if instructions[change_ptr].instruction == instruction_set.jmp {
            instructions[change_ptr].instruction = instruction_set.nop.to_string();
        }
    }
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
