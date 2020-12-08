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
    visited: bool,
}

/// Find value of accumulator immediately before any instruction is executed a second time
fn main() {
    let instruction_set: InstructionSet = InstructionSet {
        acc: "acc".to_string(),
        jmp: "jmp".to_string(),
        nop: "nop".to_string(),
    };

    let mut acc: i32 = 0;
    let mut ptr: usize = 0;

    // (instruction, visited)
    let mut instructions: Vec<Instruction> = Vec::new();
    if let Ok(lines) = _read_lines("./input.txt") {
        for line in lines {
            let word = line.unwrap();
            let instruction: Instruction = Instruction {
                instruction: word,
                visited: false,
            };
            instructions.push(instruction);
        }
    }

    loop {
        // Check whether already visited this instruction
        let curr_instruction = &instructions[ptr];
        if curr_instruction.visited {
            println!("already visited line {}, acc {}", ptr, acc);
            break;
        }
        let new_ptr: usize;

        let split_instruction = curr_instruction
            .instruction
            .split_whitespace()
            .collect::<Vec<&str>>();
        let instruction = split_instruction[0];
        let value: i32 = split_instruction[1].parse().unwrap();

        if instruction == instruction_set.acc {
            acc += value;
            new_ptr = ptr + 1;
        } else if instruction == instruction_set.jmp {
            new_ptr = ((ptr as i32) + value) as usize;
        } else if instruction == instruction_set.nop {
            new_ptr = ptr + 1;
        } else {
            panic!("invalid instruction: {}", split_instruction[0]);
        }

        instructions[ptr].visited = true;

        ptr = new_ptr;
    }
}

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
