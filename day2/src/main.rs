use anyhow::anyhow;

struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn from_str(line: &'a str) -> Result<Self, anyhow::Error> {
        let handle_opt_error = |msg| anyhow!("Bad line format: {}", msg);

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

fn main() {
    let input = include_str!("./input.txt");
    let (num_pw, num_valid_old, num_valid_new) =
        input
            .lines()
            .fold((0, 0, 0), |(total, valid_old, valid_new), this| {
                let pw =
                    Password::from_str(this).expect(&format!("Invalid password: line {}", total));
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

                (total + 1, old_validity, new_validity)
            });

    println!(
        "Part 1: There are {}/{} valid passwords",
        num_valid_old, num_pw
    );
    println!(
        "Part 2: There are {}/{} valid passwords",
        num_valid_new, num_pw
    );
}
