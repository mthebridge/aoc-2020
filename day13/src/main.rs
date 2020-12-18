fn wait_time(bus_id: u64, earliest: u64) -> u64 {
    (bus_id - (earliest % bus_id)) % bus_id
}

fn extended_euclid(a: u64, b: u64) -> (u64, i64, i64) {
    let mut first = a;
    let mut second = b;
    if a < b {
        first = b;
        second = a;
    }
    let mut first_coeff = 0i64;
    let mut prev_first = 1i64;
    let mut second_coeff = 1i64;
    let mut prev_second = 0i64;
    while second != 0 {
        let div = first / second;
        let rem = first % second;

        first = second;
        second = rem;
        let s: i64 = prev_first - (first_coeff * div as i64);
        prev_first = first_coeff;
        first_coeff = s;

        let t: i64 = prev_second - (second_coeff * div as i64);
        prev_second = second_coeff;
        second_coeff = t;
    }

    if a > b {
        (first, prev_first, prev_second)
    } else {
        (first, prev_second, prev_first)
    }
}

fn find_gold_star(bus_input: impl Iterator<Item = Option<u64>>) -> u64 {
    let buses = bus_input
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
    // This is the CRT as per https://brilliant.org/wiki/chinese-remainder-theorem/
    // Each of part2_buses has the remainder and modulus as the tuple.
    let total_product = buses
        .iter()
        .map(|&(_rem, modulus)| modulus)
        .product::<u64>();
    println!("total prodiuct = {}", total_product);
    buses
        .iter()
        .map(|&(rem, modulus)| {
            let rest_product = total_product / modulus;

            let (gcd, a, b) = extended_euclid(rest_product, modulus);
            // Verify Euclid implementation...
            assert_eq!(gcd, 1);
            assert_eq!(a * rest_product as i64 + b * modulus as i64, 1);
            let ret = (modulus as i128 - rem as i128) * a as i128 * rest_product as i128;
            println!(
                "For bus {} (rem {} rest {}) found {} and {} = {}",
                modulus, rem, rest_product, a, b, ret
            );
            let real_ret = (ret as u64 + total_product) % total_product;
            println!("Modulus ret {}", real_ret);
            real_ret
        })
        .sum::<u64>()
        % total_product
}

fn parse_bus_line(line: &str) -> impl Iterator<Item = Option<u64>> + Clone + '_ {
    line.split(',').map(|s| s.parse::<u64>().ok())
}

fn main() {
    let input = include_str!("./input.txt");
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse::<u64>().unwrap();
    let buses = parse_bus_line(lines.next().unwrap());
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

    // For part2: find the first time]

    let answer = find_gold_star(buses.clone());
    println!("Part 2; {}", answer);
}

#[test]
fn test_euclid() {
    assert_eq!(extended_euclid(240, 46), (2, -9, 47));
}

#[test]
fn test_part2_a() {
    let buses = parse_bus_line("17,x,13,19");
    assert_eq!(find_gold_star(buses), 3417);
}
#[test]
fn test_part2_b() {
    let buses = parse_bus_line("67,7,59,61");
    assert_eq!(find_gold_star(buses), 754018);
}

#[test]
fn test_part2_c() {
    let buses = parse_bus_line("1789,37,47,1889");
    assert_eq!(find_gold_star(buses), 1202161486);
}
