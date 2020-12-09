const PREAMBLE_SIZE: usize = 25;

fn find_first_invalid(input: &str) -> Option<u64> {
    let nums = input
        .lines()
        .map(|l| l.parse::<u64>().expect("Not integer"))
        .collect::<Vec<_>>();

    (PREAMBLE_SIZE..nums.len())
        .filter(|&n| {
            let target = nums[n];
            let mut candidates = nums[n - PREAMBLE_SIZE..n].to_owned();
            candidates.sort();
            // Get all pairs of all candidates, and check if any match.
            candidates
                .iter()
                .filter(|&x| candidates.iter().rev().any(|&y| x + y == target))
                .next()
                .is_none()
        })
        .map(|i| nums[i])
        .next()
}

fn main() {
    let input = include_str!("./input.txt");
    let first_invalid = find_first_invalid(input).expect("All numbers valid!");

    println!("Part 1: Bad number {}", first_invalid);
    println!("Part 2");
}
