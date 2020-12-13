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
        .filter(|&s| s != "x")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    // Want the bus whose multiple is closest to (on ar after) earliest.
    let mut part1_buses = buses.clone();
    part1_buses.sort_unstable_by_key(|&k| wait_time(k, earliest));
    let my_bus = part1_buses[0];
    let my_wait_time = wait_time(my_bus, earliest);

    println!(
        "Part 1; {} * {} = {}",
        my_bus,
        my_wait_time,
        my_bus * my_wait_time
    );
}
