use anyhow::anyhow;

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn from_str(line: &str) -> Result<Self, anyhow::Error> {
        let handle_opt_error = |msg| anyhow!("Bad line format: {}", msg);

        let mut x = line.splitn(2, ':');
        let left = x
            .next()
            .ok_or_else(|| handle_opt_error("Empty line"))?
            .trim();
        let password = x.next().ok_or_else(|| handle_opt_error("No colon"))?.trim();
        // Line has format min-max letter: password
        let mut x = left.split_whitespace();
        let counts = x.next().ok_or_else(|| handle_opt_error("Empty line"))?;
        let letter = x
            .next()
            .ok_or_else(|| handle_opt_error("Need a word after colon"))?
            .chars()
            .next()
            .ok_or_else(|| handle_opt_error("Second word should be a single char"))?;
        let mut count_vals = counts.splitn(2, '-');
        let min = count_vals
            .next()
            .ok_or_else(|| handle_opt_error("Counts empty"))
            .and_then(|s| Ok(str::parse(s)?))?;
        let max = count_vals
            .next()
            .ok_or_else(|| handle_opt_error("Counts not continaing a dash"))
            .and_then(|s| Ok(str::parse(s)?))?;

        Ok(Password {
            min,
            max,
            letter,
            password: password.trim().to_string(),
        })
    }

    fn is_valid(&self) -> bool {
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
    let (num_pw, num_valid, num_valid_new) =
        input
            .lines()
            .fold((0, 0, 0), |(total, valid_old, valid_new), this| {
                let pw =
                    Password::from_str(this).expect(&format!("Invalid password: line {}", total));
                let old_validity = if pw.is_valid() {
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

    println!("Part1: There are {}/{} valid passwords", num_valid, num_pw);
    println!(
        "Part2: There are {}/{} valid passwords",
        num_valid_new, num_pw
    );
}
