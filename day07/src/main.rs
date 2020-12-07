use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
// Name and count of each inner type
struct BagCount<'a> {
    name: &'a str,
    count: usize,
}

type BagRules<'a> = HashMap<&'a str, Vec<BagCount<'a>>>;

fn parse_input(s: &str) -> BagRules<'_> {
    s.lines()
        .map(|l| {
            let mut words = l.splitn(2, "contain");
            let outer = words
                .next()
                .expect("No word")
                .trim()
                .trim_end_matches("bags")
                .trim();
            let inners = words
                .next()
                .unwrap()
                .split(",")
                .map(|e| e.trim().trim_end_matches('.'))
                .filter_map(|entry| {
                    if entry == "no other bags" {
                        None
                    } else {
                        let mut parts = entry.splitn(2, " ");
                        let count = parts.next().unwrap().parse().unwrap();
                        Some(BagCount {
                            count,
                            name: parts
                                .next()
                                .unwrap()
                                .trim()
                                .trim_end_matches('s')
                                .trim_end_matches(" bag"),
                        })
                    }
                })
                .collect();
            (outer, inners)
        })
        .collect()
}

fn find_all_containing<'a>(rules: &'a BagRules, desc: &'a str) -> HashSet<&'a str> {
    // Search for all bags that have this type inside them, recursively.
    rules
        .iter()
        .filter_map(|(outer, inners)| {
            if inners.iter().any(|b| b.name == desc) {
                Some(outer)
            } else {
                None
            }
        })
        .flat_map(|&name| {
            println!("Checking {}", name);
            let mut set = find_all_containing(rules, name);
            set.insert(name);
            set
        })
        .collect()
}

fn count_all_inside(rules: &BagRules, desc: &str) -> usize {
    rules
        .get(desc)
        .expect("Missing bag type")
        .iter()
        // Multiply the count for this inner value by the number of bags inside each of those,
        // plus 1 for this bag itself.
        .fold(0, |sum, next| {
            sum + next.count * (count_all_inside(rules, next.name) + 1)
        })
}

fn main() {
    let input = include_str!("./input.txt");
    let rules = parse_input(input);
    let part1 = find_all_containing(&rules, "shiny gold");
    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", count_all_inside(&rules, "shiny gold"));
}
