use std::{fmt, num::ParseIntError};

struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

#[derive(Debug)]
struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse input: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

impl std::convert::From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError(e.to_string())
    }
}

impl<'a> Password<'a> {
    fn from_str(line: &'a str) -> Result<Self, ParseError> {
        let handle_opt_error = |msg| ParseError(format!("Bad line format: {}", msg));

        // Expect the first 4 things split by colon, dash or space.
        let mut components = line.splitn(4, |c| c == ':' || c == '-' || c == ' ');
        let min = str::parse(
            components
                .next()
                .ok_or_else(|| handle_opt_error("Empty line"))?
                .trim(),
        )?;
        let max = str::parse(
            components
                .next()
                .ok_or_else(|| handle_opt_error("Not enough components"))?
                .trim(),
        )?;

        let letter = components
            .next()
            .ok_or_else(|| handle_opt_error("Not enough components"))?
            .trim()
            .chars()
            .next()
            .ok_or_else(|| handle_opt_error("Second word is empty"))?;
        let password = components
            .next()
            .ok_or_else(|| handle_opt_error("Not enough components"))?
            .trim();

        Ok(Password {
            min,
            max,
            letter,
            password,
        })
    }

    fn is_valid_old(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_new(&self) -> bool {
        // Treat min/max as positions
        if self.password.len() < self.max {
            panic!("Password too short to check!");
        }
        let mut c = self.password.chars();
        // `nth` doesn't reset the iterator, hence the `max-min`.  Also the puzzle uses 1-indexing.
        let (test1, test2) = (
            c.nth(self.min - 1).unwrap(),
            c.nth(self.max - self.min - 1).unwrap(),
        );
        (test1 == self.letter && test2 != self.letter)
            || (test1 != self.letter && test2 == self.letter)
    }
}

fn get_valid_passwords(input: &str) -> (i32, i32) {
    input.lines().fold((0, 0), |(valid_old, valid_new), this| {
        let pw = Password::from_str(this).expect("Invalid password");
        let old_validity = if pw.is_valid_old() {
            valid_old + 1
        } else {
            valid_old
        };
        let new_validity = if pw.is_valid_new() {
            valid_new + 1
        } else {
            valid_new
        };

        (old_validity, new_validity)
    })
}

fn main() {
    let input = include_str!("./input.txt");
    let (num_valid_old, num_valid_new) = get_valid_passwords(input);

    println!("Part 1: There are {} valid passwords", num_valid_old);
    println!("Part 2: There are {} valid passwords", num_valid_new);
}
