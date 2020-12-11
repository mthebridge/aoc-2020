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

fn get_neighbours(
    map: &SeatMap,
    row: usize,
    col: usize,
) -> impl Iterator<Item = Option<Seat>> + std::fmt::Debug + '_ {
    (row as isize - 1..=row as isize + 1).flat_map(move |r| {
        (col as isize - 1..=col as isize + 1)
            // Ignore the seat itself
            .map(move |c| {
                if r as usize == row && c as usize == col {
                    None
                } else {
                    get_seat_if_present(map, r, c)
                }
            })
    })
}

fn run_step(map: &SeatMap) -> SeatMap {
    let rows = map.len();
    let cols = map[0].len();

    (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| match map[row][col] {
                    None => None,
                    Some(s) => {
                        let occupied_neighbours = get_neighbours(map, row, col)
                            .filter(|&n| n == Some(Seat::Occupied))
                            .count();
                        // Seat is occupied if:
                        // - was empty and all neighbours are not occupied
                        // - was occupied and less than 4 neighbours occupied
                        if (s == Seat::Empty && occupied_neighbours == 0)
                            || (s == Seat::Occupied && occupied_neighbours < 4)
                        {
                            Some(Seat::Occupied)
                        } else {
                            Some(Seat::Empty)
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let input = include_str!("./input.txt");
    let mut layout: SeatMap = input
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

    // dbg!(&layout);
    loop {
        let new_layout = run_step(&layout);
        if new_layout == layout {
            break;
        } else {
            layout = new_layout;
        }
    }

    let occ_count = layout.iter().fold(0, |sum, row| {
        sum + row.iter().filter(|&&s| s == Some(Seat::Occupied)).count()
    });

    println!("Part 1: Stable with {} seats occupied", occ_count);
}
