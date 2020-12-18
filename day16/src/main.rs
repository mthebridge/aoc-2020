use std::collections::{HashMap, HashSet};

struct TicketData<'a> {
    rules: HashMap<&'a str, HashSet<u32>>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
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
                    valids.insert(v as u32);
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
    let valids: HashSet<u32> = tickets.rules.values().fold(HashSet::new(), |set, this| {
        set.extend(this);
        set
    });
    let invalids = tickets
        .nearby_tickets
        .iter()
        .map(|ticket| ticket.iter().filter(|&val| valids.get(val).is_none()))
        .flatten();
    println!("Part1: {}", invalids.sum::<u32>());
}
