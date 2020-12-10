fn get_differences(voltages: &[u32]) -> impl Iterator<Item = u32> + Clone + '_ {
    (1..voltages.len()).map(move |idx| {
        let diff = voltages[idx] - voltages[idx - 1];
        if diff < 1 || diff > 3 {
            panic!("Invalid difference")
        };
        diff
    })
}

fn main() {
    let input = include_str!("./input.txt");

    let mut voltages: Vec<u32> = input
        .lines()
        .map(|s| s.parse().expect("Not integer"))
        .collect();

    voltages.sort_unstable();
    // Add the zero-index.
    voltages.insert(0, 0);

    let diffs = get_differences(&voltages);

    let ones = diffs.clone().filter(|&d| d == 1).count();
    let threes = diffs.filter(|&d| d == 3).count() + 1;

    println!(
        "Part 1: {} one-diffs * {} three-diffs = {}",
        ones,
        threes,
        ones * threes
    );

    // Ugh.

    let min_length = voltages.len() / 3;

    println!("Part 2");
}
