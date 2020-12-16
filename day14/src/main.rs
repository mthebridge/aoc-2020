use std::collections::HashMap;

fn run_initialization(input: &str) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut cur_mask = HashMap::new();
    for line in input.lines() {
        match line.split_whitespace().next() {
            Some("mask") => {
                cur_mask = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(idx, c)| match c {
                        'X' => None,
                        '1' => Some((idx, true)),
                        '0' => Some((idx, false)),
                        _ => panic!("Bad mask"),
                    })
                    .collect();
            }
            Some(x) if x.starts_with("mem") => {
                // Format is mem[addr] = val
                let addr = x
                    .trim_start_matches("mem[")
                    .trim_end_matches("]")
                    .parse::<u64>()
                    .unwrap();
                let val = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                // Mask the val
                let masked = cur_mask.iter().fold(val, |acc, (&idx, &v)| {
                    if v {
                        // Set this bit to 1
                        acc | (1 << idx)
                    } else {
                        // Clear this bit
                        acc & !(1 << idx)
                    }
                });
                memory.insert(addr, masked);
            }
            _ => panic!("Invalid line"),
        }
    }
    memory
}

fn main() {
    let input = include_str!("./input.txt");
    let mem = run_initialization(input);

    let sum: u64 = mem.values().sum();
    println!("Part 1: {}", sum);
}
