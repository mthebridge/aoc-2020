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
    let mut part2_buses = buses
        .clone()
        .enumerate()
        .filter_map(|(idx, bus)| {
            if let Some(b) = bus {
                println!("Must be {} modulo {}", idx, b);
                Some((idx, b))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    part2_buses.sort_unstable_by_key(|&k| k.0);

    // // Need to find the first value that appears in each
    // // Let candidates
    // while let next = part2_buses.pop() {
    //     match next {
    //         None => return ,
    //         Some((idx, val)) => {
    //             for v in (idx..).step_by(val) {
    //                 // This value satisfies the current constraint.  Recurse.

    //             }
    //         }
    //     }

    // }

    // println!("Part 2; {}", part2_buses.iter().next().unwrap());
}
