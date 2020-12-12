#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    West,
    South,
}

impl Dir {
    fn turn(&self, angle: u32, left: bool) -> Self {
        match *self {
            Dir::North => match angle {
                90 => {
                    if left {
                        Dir::West
                    } else {
                        Dir::East
                    }
                }
                180 => Dir::South,
                270 => {
                    if left {
                        Dir::East
                    } else {
                        Dir::West
                    }
                }
                _ => panic!("invalid angle"),
            },
            Dir::South => match angle {
                90 => {
                    if left {
                        Dir::East
                    } else {
                        Dir::West
                    }
                }
                180 => Dir::North,
                270 => {
                    if left {
                        Dir::West
                    } else {
                        Dir::East
                    }
                }
                _ => panic!("invalid angle"),
            },
            Dir::East => match angle {
                90 => {
                    if left {
                        Dir::North
                    } else {
                        Dir::South
                    }
                }
                180 => Dir::West,
                270 => {
                    if left {
                        Dir::South
                    } else {
                        Dir::North
                    }
                }
                _ => panic!("invalid angle"),
            },
            Dir::West => match angle {
                270 => {
                    if left {
                        Dir::North
                    } else {
                        Dir::South
                    }
                }
                180 => Dir::East,
                90 => {
                    if left {
                        Dir::South
                    } else {
                        Dir::North
                    }
                }
                _ => panic!("invalid angle"),
            },
        }
    }
}

fn get_final_posn_part1(input: &str) -> (i64, i64) {
    let mut xpos = 0i64;
    let mut ypos = 0i64;
    let mut curdir = Dir::East;
    for l in input.lines() {
        let instr = l.chars().next().unwrap();
        let arg = l[1..].parse::<u32>().expect("Invalid integer");

        match instr {
            'N' => ypos += i64::from(arg),
            'E' => xpos += i64::from(arg),
            'S' => ypos -= i64::from(arg),
            'W' => xpos -= i64::from(arg),
            'F' if curdir == Dir::North => ypos += i64::from(arg),
            'F' if curdir == Dir::East => xpos += i64::from(arg),
            'F' if curdir == Dir::South => ypos -= i64::from(arg),
            'F' if curdir == Dir::West => xpos -= i64::from(arg),
            'L' => curdir = curdir.turn(arg, true),
            'R' => curdir = curdir.turn(arg, false),
            _ => panic!("Invalid instruction"),
        }
    }

    (xpos, ypos)
}

fn get_final_posn_part2(input: &str) -> (i64, i64) {
    let mut xpos = 0i64;
    let mut ypos = 0i64;
    // Waypoint posn is relative to ship.
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    for l in input.lines() {
        let instr = l.chars().next().unwrap();
        let arg = l[1..].parse::<u32>().expect("Invalid integer");

        match instr {
            'N' => waypoint_y += i64::from(arg),
            'E' => waypoint_x += i64::from(arg),
            'S' => waypoint_y -= i64::from(arg),
            'W' => waypoint_x -= i64::from(arg),
            'F' => {
                ypos += i64::from(arg) * waypoint_y;
                xpos += i64::from(arg) * waypoint_x;
            }
            'L' => {
                // Eg (1, 2) => (-2, 1)

                for _ in 0..(arg / 90) {
                    let (new_x, new_y) = (-waypoint_y, waypoint_x);
                    waypoint_x = new_x;
                    waypoint_y = new_y;
                }
            }
            'R' => {
                for _ in 0..(arg / 90) {
                    // Eg (1, 2) => (2, -1)
                    let (new_x, new_y) = (waypoint_y, -waypoint_x);
                    waypoint_x = new_x;
                    waypoint_y = new_y;
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }

    (xpos, ypos)
}

fn main() {
    let input = include_str!("./input.txt");
    let (x, y) = get_final_posn_part1(input);
    println!("Part 1: At ({}, {}), sum {}", x, y, x.abs() + y.abs());

    let (x, y) = get_final_posn_part2(input);
    println!("Part 2: At ({}, {}), sum {}", x, y, x.abs() + y.abs());
}
