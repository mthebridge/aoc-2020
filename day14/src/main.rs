use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ver {
    V1,
    V2,
}

fn apply_mask(mask: &[Option<bool>], base: u64) -> u64 {
    mask.iter().enumerate().fold(base, |acc, (idx, &v)| {
        match v {
            // Set this bit to 1
            Some(true) => acc | (1 << idx),
            // Clear this bit
            Some(false) => acc & !(1 << idx),
            // Ignore anything else
            None => acc,
        }
    })
}

fn run_initialization(input: &str, version: Ver) -> HashMap<u64, u64> {
    let mut memory = HashMap::new();
    let mut cur_mask = Vec::with_capacity(36);
    let mut masks = Vec::new();
    for line in input.lines() {
        match line.split_whitespace().next() {
            Some("mask") => {
                cur_mask = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .chars()
                    .rev()
                    .map(|c| match c {
                        'X' => None,
                        '1' => Some(true),
                        '0' => Some(false),
                        _ => panic!("Bad mask"),
                    })
                    .collect();

                if version == Ver::V2 {
                    // Generate all possible masks.
                    masks = vec![];
                    masks.push(cur_mask.clone());
                    for (idx, val) in cur_mask.iter().enumerate() {
                        for mask in &mut masks {
                            mask[idx] = match val {
                                Some(true) => Some(true),
                                Some(false) => None,
                                None => Some(false),
                            }
                        }
                        if val.is_none() {
                            let mut new_masks = masks.clone();
                            for mask in &mut masks {
                                mask[idx] = Some(true);
                            }
                            for mask in &mut new_masks {
                                mask[idx] = Some(false);
                            }
                            masks.extend(new_masks.into_iter());
                        }
                    }
                }
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
                match version {
                    Ver::V1 => {
                        let masked = apply_mask(&cur_mask, val);
                        memory.insert(addr, masked);
                    }
                    Ver::V2 => {
                        for mask in &masks {
                            let masked = apply_mask(&mask, addr);
                            memory.insert(masked, val);
                        }
                    }
                }
            }
            _ => panic!("Invalid line"),
        }
    }
    memory
}

fn main() {
    let input = include_str!("./input.txt");
    let mem = run_initialization(input, Ver::V1);
    let sum: u64 = mem.values().sum();
    println!("Part 1: {}", sum);

    let mem = run_initialization(input, Ver::V2);
    let sum: u64 = mem.values().sum();
    println!("Part 2: {}", sum);
}

#[test]
fn test_part2() {
    let input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
    let mem = run_initialization(input, Ver::V2);
    let sum: u64 = mem.values().sum();
    assert_eq!(sum, 208);
}
