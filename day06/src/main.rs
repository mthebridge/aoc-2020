use std::collections::HashSet;

// Gets all questions to which someone answerre "yes"
fn get_all_answers(group_data: &str) -> HashSet<char> {
    group_data.replace("\n", "").chars().collect::<HashSet<_>>()
}

fn count_answers_from_all(group_data: &str) -> usize {
    let answers = get_all_answers(group_data);
    answers
        .iter()
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
