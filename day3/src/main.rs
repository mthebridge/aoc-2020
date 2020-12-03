use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    trees: HashSet<(usize, usize)>,
}

impl Map {
    fn from_str(input: &str) -> Result<Self, anyhow::Error> {
        let mut trees = HashSet::new();
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => (),
                    '#' => {
                        if !trees.insert((x, y)) {
                            panic!("Duplicate coordinates?")
                        }
                    }
                    _ => Err(anyhow::anyhow!("Invalid character {} in input", ch))?,
                }
                if height == 0 {
                    width = x + 1
                }
            }
            height = y + 1;
        }
        Ok(Map {
            height,
            width,
            trees,
        })
    }

    fn count_trees_on_slope(&self, x_change: usize, y_change: usize) -> usize {
        (0..)
            .step_by(x_change)
            .zip((0..self.height).step_by(y_change))
            .fold(0, |count, this| {
                if self.trees.contains(&(this.0 % self.width, this.1)) {
                    println!("Hit tree at ({}, {})", this.0, this.1);
                    count + 1
                } else {
                    count
                }
            })
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let map = Map::from_str(input).expect("Failed to parse input!");
    dbg!(&map.height);
    dbg!(&map.width);
    println!("Part 1: {} trees on path", map.count_trees_on_slope(3, 1));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let map = Map::from_str(input).expect("Failed to parse input!");
        dbg!(&map.height);
        dbg!(&map.width);
        let trees = map.count_trees_on_slope(3, 1);
        assert_eq!(trees, 7);
    }
}
