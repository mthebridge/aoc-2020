fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| str::parse(s).expect("Could not parse input line!"))
        .collect()
}

fn find_pair_with_sum(i: &[u32], target: u32) -> Option<(u32, u32)> {
    for (idx, first) in i.iter().enumerate() {
        for second in &i[idx + 1..] {
            if (first + second) == target {
                return Some((*first, *second));
            }
        }
    }
    None
}

fn find_triple_with_sum(i: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    for (idx, first) in i.iter().enumerate() {
        for second in &i[idx + 1..] {
            for third in &i[idx + 2..] {
                if (first + second + third) == target {
                    return Some((*first, *second, *third));
                }
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("./input.txt");
    let expenses = parse_input(input);
    let (a, b) = find_pair_with_sum(&expenses, 2020).expect("No valid answers");
    println!("Part 1: {} * {} = {}", a, b, a * b);

    let (a, b, c) = find_triple_with_sum(&expenses, 2020).expect("No valid answers");
    println!("Part 2: {} * {} * {} = {}", a, b, c, a * b * c);
}
