use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
struct BoardingPass {
    row: u8,
    seat: u8,
}

impl BoardingPass {
    fn from_str(input: &str) -> Self {
        if input.len() != 10 {
            panic!("Invalid input line - too long");
        }
        let (rows, seat) = input.split_at(7);

        // Turn the sequences of letters into a row or seat number.
        // Effectively these are binary numbers.  So :
        //  - map the letters to 1/0 for an iterator of binary digits;
        //  - reverse the iterator so we start at the units end;
        // -  fold the iterator, adding increasing powers of two at each stage.
        let bin_parser = |s: &str, lower: char, upper: char| -> u8 {
            s.chars()
                .map(|c| match c {
                    x if x == lower => 0,
                    x if x == upper => 1,
                    _ => panic!("Invalid input char"),
                })
                .rev()
                .enumerate()
                .fold(0u8, |acc, (idx, next)| {
                    acc + (next * 2u8.pow(u32::try_from(idx).unwrap()))
                })
        };

        BoardingPass {
            row: bin_parser(rows, 'F', 'B'),
            seat: bin_parser(seat, 'L', 'R'),
        }
    }

    fn seat_id(&self) -> u16 {
        u16::from(self.row) * 8 + u16::from(self.seat)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let get_seat_ids = || input.lines().map(|l| BoardingPass::from_str(l).seat_id());
    let max_id = get_seat_ids().max().expect("Empty list");
    println!("Part 1: Max Seat ID {}", max_id);
    let mut seat_ids = get_seat_ids().collect::<Vec<_>>();
    // Find the missing ID.
    // Easiest way to do this is sort the list of IDs, then walk them looking for the first
    // where the previous entry is not one lower.
    // That entry in the seat_ids vector is one higher than the missing ID.
    seat_ids.sort_unstable();
    let mut candidate_indices = (1..seat_ids.len())
        .filter(|&idx| seat_ids[idx] >= 8 && seat_ids[idx - 1] != seat_ids[idx] - 1);
    let next_seat_idx = candidate_indices.next().expect("No missing seat");
    // Sanity check there was only one free seat...
    assert!(candidate_indices.next().is_none());
    let my_seat = seat_ids[next_seat_idx] - 1;
    println!("Part 2: Missing Seat ID: {}", my_seat);
}
