use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    trees: HashSet<(usize, usize)>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let lines = input.lines();

        let trees = lines
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, ch)| match ch {
                        '.' => None,
                        '#' => Some((x, y)),
                        _ => panic!("Invalid character {} in input", ch),
                    })
            })
            .flatten()
            .collect();

        let mut lines = input.lines();
        Map {
            width: lines.next().unwrap().chars().count(),
            height: lines.count() + 1,
            trees,
        }
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
    let map = Map::from_str(input);
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
        let trees = Map::from_str(input).count_trees_on_slope(3, 1);
        assert_eq!(trees, 7);
    }
}
