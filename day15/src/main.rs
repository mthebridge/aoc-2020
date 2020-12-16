use std::collections::HashMap;

fn run_game_until_step(starting: &[usize], turns: usize) -> usize {
    let start_size = starting.len();
    let mut last_val = starting[start_size - 1];
    let mut data = starting[..start_size - 1]
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect::<HashMap<_, _>>();

    for idx in starting.len()..turns {
        // Find the last time the previous number was spoken
        let prev = data.entry(last_val).or_insert(idx - 1);
        last_val = idx - 1 - *prev;
        *prev = idx - 1;
    }
    last_val
}

fn main() {
    let starting_numbers: [usize; 7] = [11, 18, 0, 20, 1, 7, 16];
    println!("Part1: {}", run_game_until_step(&starting_numbers, 2020));
    println!(
        "Part2: {}",
        run_game_until_step(&starting_numbers, 30_000_000)
    );
}

#[test]
fn test_sample() {
    assert_eq!(run_game_until_step(&[0, 3, 6], 9), 4);
    assert_eq!(run_game_until_step(&[0, 3, 6], 2020), 436);
}
