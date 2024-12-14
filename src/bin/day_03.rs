use regex::Regex;

mod utils;

#[derive(Debug, Clone, Eq, PartialEq)]
enum InstructionType {
    Multiply,
    Do,
    Dont,
}

#[derive(Debug, Clone)]
struct Instruction {
    instruction_type: InstructionType,
    arg1: i64,
    arg2: i64,
}

pub fn main() {
    let instructions = parse_input();
    solve_part1(&instructions);
    solve_part2(&instructions);
}

fn solve_part1(instructions: &Vec<Instruction>) {
    let multiply_instructions: Vec<Instruction> = instructions
        .iter()
        .filter(|inst| inst.instruction_type == InstructionType::Multiply)
        .cloned()
        .collect();
    println!("Part 1: {}", run_instructions(&multiply_instructions));
}

fn solve_part2(instructions: &Vec<Instruction>) {
    println!("Part 2: {}", run_instructions(instructions));
}

fn run_instructions(instructions: &Vec<Instruction>) -> i64 {
    let mut sum: i64 = 0;
    let mut enabled: bool = true;
    instructions
        .iter()
        .for_each(|inst| match inst.instruction_type {
            InstructionType::Multiply => {
                if enabled {
                    sum += inst.arg1 * inst.arg2;
                }
            }
            InstructionType::Do => enabled = true,
            InstructionType::Dont => enabled = false,
        });
    return sum;
}

fn parse_input() -> Vec<Instruction> {
    let inst_re = Regex::new(r"(mul|do|don't)\(([\d,]*)\)").unwrap();
    let args_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let lines = utils::read_puzzle_input();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        inst_re.captures_iter(&line).for_each(|inst_cap| {
            let inst_str = inst_cap.get(1).unwrap().as_str();
            let inst_type = match inst_str {
                "mul" => InstructionType::Multiply,
                "do" => InstructionType::Do,
                "don't" => InstructionType::Dont,
                _ => panic!("Invalid instruction type {}", inst_str),
            };

            match inst_type {
                InstructionType::Multiply => {
                    let args_cap = args_re
                        .captures(&inst_cap.get(2).unwrap().as_str())
                        .unwrap();
                    instructions.push(Instruction {
                        instruction_type: inst_type,
                        arg1: args_cap.get(1).unwrap().as_str().parse().unwrap(),
                        arg2: args_cap.get(2).unwrap().as_str().parse().unwrap(),
                    });
                }
                InstructionType::Do | InstructionType::Dont => instructions.push(Instruction {
                    instruction_type: inst_type,
                    arg1: 0,
                    arg2: 0,
                }),
            }
        });
    }
    return instructions;
}
