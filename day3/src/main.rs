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
                    _ => return Err(anyhow::anyhow!("Invalid character {} in input", ch)),
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
                count + 1
            } else {
                count
            }
        })
    }
}

const ADDITIONAL_SLOPES: [(usize, usize); 4] = [(1, 1), (5, 1), (7, 1), (1, 2)];

fn main() {
    let input = include_str!("./input.txt");
    let map = Map::from_str(input).expect("Failed to parse input!");
    let part1_treecount = map.count_trees_on_slope(3, 1);
    println!("Part 1: {} trees on path", part1_treecount);
    let part2_product = ADDITIONAL_SLOPES
        .iter()
        .fold(part1_treecount, |product, (x, y)| {
            product * map.count_trees_on_slope(*x, *y)
        });
    println!("Part 2: Product of all five answers: {}", part2_product);
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
        let trees = map.count_trees_on_slope(3, 1);
        assert_eq!(trees, 7);
    }
}
