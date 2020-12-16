fn wait_time(bus_id: u32, earliest: u32) -> u32 {
    (bus_id - (earliest % bus_id)) % bus_id
}

fn main() {
    let input = include_str!("./input.txt");
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse::<u32>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().ok());
    // Want the bus whose multiple is closest to (on ar after) earliest.
    let mut part1_buses = buses.clone().filter_map(|b| b).collect::<Vec<_>>();
    part1_buses.sort_unstable_by_key(|&k| wait_time(k, earliest));
    let my_bus = part1_buses[0];
    let my_wait_time = wait_time(my_bus, earliest);

    println!(
        "Part 1; {} * {} = {}",
        my_bus,
        my_wait_time,
        my_bus * my_wait_time
    );

    // For [aprt2: find the first time]
    let mut part2_buses = buses.clone().
    enumerate().filter_map(|(idx, bus)| {
        if let Some(b) = bus {
            println!("Must be {} modulo {}", idx, b);
            Some((idx, b))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    part2_buses.sort_unstable_by_key(|&k| k.1);

    type Candidate = Box<dyn Iterator<Item=usize>>;
    let mut candidates: Option<Candidate> = None;
    for r in part2_buses.rev() {
        if let Some(existing) = candidates {
            Some(Box::new(existing.filter(|c| r.(c).is_some()))as Candidate)
        } else {
            Some(Box::new(r.map(|(_, v)|)) as Candidate)
        }
    }

    // Need to find the first value that appears in each


    println!("Part 2; {}", candidates.unwrap().next().unwrap());
}
