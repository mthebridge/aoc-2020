use std::collections::HashSet;

enum Instruction {
    Nop(i32),
    Acc(i32),
    Jump(i32),
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

fn run_until_loop(instructions: &[Instruction]) -> i32 {
    let mut acc = 0;
    let mut idx = 0;
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(idx) {
            return acc;
        }
        match instructions[idx] {
            Instruction::Nop(_) => idx += 1,
            Instruction::Jump(arg) => {
                if arg > 0 {
                    idx += arg as usize
                } else {
                    idx -= (-arg) as usize
                }
            }
            Instruction::Acc(arg) => {
                acc += arg;
                idx += 1
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let instructions = input.lines().map(parse_line).collect::<Vec<_>>();
    println!(
        "Part 1: Acc is {} after repeat",
        run_until_loop(&instructions)
    );
    println!("Part 2");
}
