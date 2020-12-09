use std::collections::HashSet;

enum Instruction {
    Nop(i16),
    Acc(i16),
    Jump(i16),
}

fn parse_line(line: &str) -> Instruction {
    let mut words = line.split_whitespace();
    let itype = words.next().expect("Empty line");
    let arg = words
        .next()
        .expect("No argument")
        .parse()
        .expect("Invalid arg");
    assert!(words.next().is_none());
    match itype {
        "acc" => Instruction::Acc(arg),
        "jmp" => Instruction::Jump(arg),
        "nop" => Instruction::Nop(arg),
        _ => panic!("invalid line"),
    }
}

fn run_until_loop(instructions: &[Instruction], patch_idx: Option<usize>) -> (bool, i32) {
    let mut acc = 0;
    let mut idx = 0;
    let mut seen = HashSet::new();

    loop {
        // If we're about to look off the end of the instructions, return success.
        if idx >= instructions.len() {
            return (true, acc);
        }
        // We've been here before - end infinite loop
        if !seen.insert(idx) {
            return (false, acc);
        }
        match instructions[idx] {
            Instruction::Jump(arg) => {
                if Some(idx) == patch_idx {
                    idx += 1
                } else {
                    idx = idx.wrapping_add(arg as usize);
                }
            }
            Instruction::Acc(arg) => {
                acc += arg;
                idx += 1
            }
            Instruction::Nop(arg) => {
                if Some(idx) == patch_idx {
                    idx = idx.wrapping_add(arg as usize);
                } else {
                    idx += 1
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let instructions = input.lines().map(parse_line).collect::<Vec<_>>();
    let (terminated, acc) = run_until_loop(&instructions, None);
    assert!(!terminated);
    println!("Part 1: Acc is {} after repeat", acc);

    for idx in 0..instructions.len() {
        // Try changing each index
        let (terminated, acc) = run_until_loop(&instructions, Some(idx));
        if terminated {
            println!("Part 2: Acc is {} after changing instruction {}", acc, idx);
            break;
        }
    }
}
