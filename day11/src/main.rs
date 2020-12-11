#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

type SeatMap = Vec<Vec<Option<Seat>>>;

fn get_seat_if_present(map: &SeatMap, row: isize, col: isize) -> Option<Seat> {
    if row < 0 || col < 0 || row as usize >= map.len() || col as usize >= map[0].len() {
        None
    } else {
        map[row as usize][col as usize]
    }
}

// Tuples representing the 8 cardinal directions.
const DIRS: [(isize, isize); 8] = [
    (-1, -1), // NW
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, -1),  // W
    (0, 1),   // E
    (1, -1),  // SW
    (1, 0),   // S
    (1, 1),   // SE
];

// How many directly adjacent seats are occupied.
fn get_neighbour_occupied_count(map: &SeatMap, row: usize, col: usize) -> usize {
    DIRS.iter()
        .map(move |&(r, c)| get_seat_if_present(map, row as isize + r, col as isize + c))
        .filter(|&n| n == Some(Seat::Occupied))
        .count()
}

// How many of the visible seats in each direction are occupied?
fn get_visible_occupied_neighbour_count(
    map: &SeatMap,
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> usize {
    // Repeat for each direction.
    DIRS.iter()
        .map(move |&(r, c)| {
            // Could be faster by calculating the max for each direction, but life is too short.
            // In release mode this
            let max_mult = [
                row,
                col,
                (max_row as isize - row as isize).abs() as usize,
                (max_col as isize - col as isize).abs() as usize,
            ]
            .iter()
            .max()
            .unwrap()
            .to_owned()
                + 1;
            (1..max_mult).find_map(|mult| {
                get_seat_if_present(
                    map,
                    row as isize + (r * mult as isize),
                    col as isize + (c * mult as isize),
                )
            })
        })
        .filter(|&n| n == Some(Seat::Occupied))
        .count()
}

fn run_step(map: &SeatMap, part2: bool) -> SeatMap {
    let rows = map.len();
    let cols = map[0].len();

    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| {
                    // println!("Checking ({},{})", row, col);
                    match map[row][col] {
                        None => None,
                        Some(s) => {
                            let occupied_neighbours = if part2 {
                                get_visible_occupied_neighbour_count(map, row, col, rows, cols)
                            } else {
                                get_neighbour_occupied_count(map, row, col)
                            };
                            // Seat is occupied if:
                            // - was empty and all neighbours are not occupied
                            // - was occupied and less than 4 neighbours occupied (5 in part2)
                            if (s == Seat::Empty && occupied_neighbours == 0)
                                || (s == Seat::Occupied
                                    && occupied_neighbours < (if part2 { 5 } else { 4 }))
                            {
                                Some(Seat::Occupied)
                            } else {
                                Some(Seat::Empty)
                            }
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn get_occupied_count(mut layout: SeatMap, part2: bool) -> usize {
    for step in 1.. {
        // println!("Step {}, part2? {}", step, part2);
        let new_layout = run_step(&layout, part2);
        if new_layout == layout {
            println!("Match after {} steps", step);
            break;
        } else {
            layout = new_layout;
        }
    }

    layout.iter().fold(0, |sum, row| {
        sum + row.iter().filter(|&&s| s == Some(Seat::Occupied)).count()
    })
}

fn main() {
    let input = include_str!("./input.txt");
    let layout: SeatMap = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    'L' => Some(Seat::Empty),
                    '#' => Some(Seat::Occupied),
                    _ => panic!("Bad input"),
                })
                .collect()
        })
        .collect();

    println!(
        "Part 1: Stable with {} seats occupied",
        get_occupied_count(layout.clone(), false)
    );
    println!(
        "Part 2: Stable with {} seats occupied",
        get_occupied_count(layout, true)
    );
}
