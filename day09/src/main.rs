const PREAMBLE_SIZE: usize = 25;

fn find_first_invalid(nums: &[u64]) -> Option<u64> {
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

fn find_sequence_summing_to(nums: &[u64], target: u64) -> &[u64] {
    // We want a continuous sequence that sums to target.
    // So:
    // * Start at the beginning
    // * Try adding on each number sequentially.
    //
    let mut sum = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    while sum != target && end_idx < nums.len() {
        if sum < target {
            sum += nums[end_idx];
            end_idx += 1;
        } else {
            sum -= nums[start_idx];
            start_idx += 1;
        }
    }
    &nums[start_idx..end_idx]
}

fn main() {
    let input = include_str!("./input.txt");
    let nums = input
        .lines()
        .map(|l| l.parse::<u64>().expect("Not integer"))
        .collect::<Vec<_>>();
    let first_invalid = find_first_invalid(&nums).expect("All numbers valid!");

    println!("Part 1: Bad number {}", first_invalid);

    let sequence = find_sequence_summing_to(&nums, first_invalid);
    assert!(sequence.iter().sum::<u64>() == first_invalid);
    let (min, max) = (
        sequence.iter().min().unwrap(),
        sequence.iter().max().unwrap(),
    );
    println!("Part 2: {} + {} = {}", min, max, min + max);
}
