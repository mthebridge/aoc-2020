use std::collections::HashSet;

// Gets all questions to which someone answered "yes"
fn get_all_answers(group_data: &str) -> HashSet<char> {
    group_data.replace("\n", "").chars().collect::<HashSet<_>>()
}

// Count the questions to which *all* members of the group answered "yes"
fn count_answers_from_all(group_data: &str) -> usize {
    get_all_answers(group_data)
        .iter()
        // We want to count this answer only if it appears in *all* lines.
        .filter(|&&c| group_data.lines().all(|l| l.contains(c)))
        .count()
}

fn main() {
    let input = include_str!("./input.txt");
    let total_uniques: usize = input.split("\n\n").map(|s| get_all_answers(s).len()).sum();
    println!("Part 1: Total {} unique answers", total_uniques);
    let total_all_answered: usize = input.split("\n\n").map(count_answers_from_all).sum();
    println!("Part 2: Total {} fully answered", total_all_answered);
}
