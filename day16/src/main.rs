use std::{
    collections::{HashMap, HashSet}
};

struct TicketData<'a> {
    rules: HashMap<&'a str, HashSet<u64>>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

fn parse_input(input: &str) -> TicketData {
    let mut rules = HashMap::new();
    let mut nearby_tickets = vec![];
    let mut my_ticket = vec![];
    let mut section = 1;
    for line in input.lines() {
        if line.len() == 0 {
            section += 1;
            continue;
        }

        if section == 1 {
            // Field rule
            let mut valids = HashSet::new();
            let name = line.split(':').next().unwrap();
            let data = line.split(':').nth(1).unwrap().split_whitespace();
            for rule in data {
                if rule == "or" {
                    continue;
                }
                let low = rule.split('-').next().unwrap().parse::<usize>().unwrap();
                let high = rule.split('-').nth(1).unwrap().parse::<usize>().unwrap();
                for v in low..=high {
                    valids.insert(v as u64);
                }
            }

            rules.insert(name, valids);
        } else if section == 2 {
            if line == "your ticket:" {
                continue;
            }
            my_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
        } else if section == 3 {
            if line == "nearby tickets:" {
                continue;
            }
            let this_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
            nearby_tickets.push(this_ticket);
        }
    }
    TicketData {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let tickets = parse_input(input);
    // Get all possible valid values for fields
    let valids: HashSet<u64> = tickets
        .rules
        .values()
        .fold(HashSet::new(), |mut set, this| {
            set.extend(this);
            set
        });
    let invalids = tickets
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.iter().filter(|&val| valids.get(val).is_none()))
        .flatten();
    println!("Part1: {}", invalids.sum::<u64>());

    let valid_tickets = tickets
        .nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&val| valids.get(&val).is_some()))
        .chain(std::iter::once(&tickets.my_ticket));

    // Need to work out which field is which.
    // Get the possible names for each field.
    let field_count = tickets.rules.len();
    let mut name_mapping = vec![""; field_count];
    let mut unknown_fields = tickets.rules.clone();

    loop {
        for idx in 0..field_count {
            let field_values = valid_tickets.clone().map(|t| t[idx]).collect::<HashSet<_>>();
            let mut candidates = vec![];
            for (&k, v) in &unknown_fields {
                if v.is_superset(&field_values) {
                    candidates.push(k);
                }
            }

            // Check if we have options.
            if candidates.len() == 1 {
                // This is the match!
                let this_key = candidates[0];
                unknown_fields.remove(&this_key).unwrap();
                name_mapping[idx] = this_key;
            }
        };

        if unknown_fields.is_empty() {
            break;
        }
    }

    // Find the indices we care about.
    dbg!(&name_mapping);
    let part2 = name_mapping
        .iter()
        .enumerate()
        .filter(|(_, &n)| n.starts_with("departure"))
        .map(|(i, _)| tickets.my_ticket[i])
        .product::<u64>();
    println!("Part2: {}", part2);
}
