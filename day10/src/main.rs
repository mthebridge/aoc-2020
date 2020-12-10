use std::collections::HashMap;

fn get_differences(voltages: &[u32]) -> impl Iterator<Item = u32> + Clone + '_ {
    (1..voltages.len()).map(move |idx| {
        let diff = voltages[idx] - voltages[idx - 1];
        if diff < 1 || diff > 3 {
            panic!("Invalid difference")
        };
        diff
    })
}

fn count_valid_subsequences(voltages: &[u32], cache: &mut HashMap<Vec<u32>, u64>) -> u64 {
    if voltages.len() <= 2 {
        return 1;
    }

    (1..voltages.len() - 1).fold(1, move |perms, idx| {
        if voltages[idx + 1] - voltages[idx - 1] <= 3 {
            // This one is skippable.  Remove from the set, and add permutations for subset.
            let mut subset = voltages[idx + 1..].to_vec();
            subset.insert(0, voltages[idx - 1]);
            perms
                + if let Some(&p) = cache.get(&subset) {
                    p
                } else {
                    let p = count_valid_subsequences(&subset, cache);
                    cache.insert(subset, p);
                    p
                }
        } else {
            perms
        }
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
    let mut cache = HashMap::new();
    let ones = diffs.clone().filter(|&d| d == 1).count();
    let threes = diffs.filter(|&d| d == 3).count() + 1;

    println!(
        "Part 1: {} one-diffs * {} three-diffs = {}",
        ones,
        threes,
        ones * threes
    );

    // Ugh.
    println!(
        "Part 2: {}",
        count_valid_subsequences(&voltages, &mut cache)
    );
}

#[test]
fn test_part2() {
    let mut testset1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    testset1.sort_unstable();
    testset1.insert(0, 0);
    let mut cache = HashMap::new();
    assert_eq!(count_valid_subsequences(&testset1, &mut cache), 8);

    let mut testset2 = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    testset2.sort_unstable();
    testset2.insert(0, 0);
    let mut cache = HashMap::new();
    assert_eq!(count_valid_subsequences(&testset2, &mut cache), 19208);
}
