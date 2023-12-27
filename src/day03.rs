// https://adventofcode.com/2019/day/3

use std::collections::HashSet;

type Coord = (isize, isize);

pub struct Input {
    wire1: Wire,
    wire2: Wire,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let wire1: Wire = lines.next().unwrap().into();
        let wire2: Wire = lines.next().unwrap().into();

        Self { wire1, wire2 }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("invalid Direction char: {c}"),
        }
    }
}

struct Wire {
    coords: Vec<Coord>,
    set: HashSet<Coord>,
}

impl Wire {
    fn intersection<'a>(&'a self, other: &'a Wire) -> HashSet<&'a Coord> {
        self.set.intersection(&other.set).collect()
    }

    fn path(&self, point: &Coord) -> isize {
        self.coords
            .iter()
            .enumerate()
            .find_map(|(path, coord)| {
                if coord == point {
                    // Need to add one because the path starts at 1, while indices start at 0
                    Some(path as isize + 1)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        let mut coords = Vec::new();
        let mut curr_coord = (0, 0);
        for segment in value.split(',') {
            let dir: Direction = segment[..1].into();
            let len = segment[1..].parse().unwrap();
            let mut coord_step = || {
                match dir {
                    Direction::Up => curr_coord.0 += 1,
                    Direction::Down => curr_coord.0 -= 1,
                    Direction::Left => curr_coord.1 -= 1,
                    Direction::Right => curr_coord.1 += 1,
                }
                curr_coord
            };
            coords.extend((0..len).map(|_| coord_step()));
            curr_coord = *coords.last().unwrap();
        }
        let set: HashSet<Coord> = coords.iter().copied().collect();
        Self { coords, set }
    }
}

fn manhattan_distance(coord: &Coord) -> isize {
    coord.0.abs() + coord.1.abs()
}

pub fn part1(input: &Input) -> isize {
    input
        .wire1
        .intersection(&input.wire2)
        .into_iter()
        .filter(|coord| **coord != (0, 0))
        .map(manhattan_distance)
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> isize {
    input
        .wire1
        .intersection(&input.wire2)
        .into_iter()
        .map(|i| input.wire1.path(i) + input.wire2.path(i))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        const INPUT: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

        assert_eq!(part1(&INPUT.into()), 159);
    }

    #[test]
    fn test_part1_ex2() {
        const INPUT: &str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

        assert_eq!(part1(&INPUT.into()), 135);
    }

    #[test]
    fn test_part2_ex1() {
        const INPUT: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

        assert_eq!(part2(&INPUT.into()), 610);
    }

    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

        assert_eq!(part2(&INPUT.into()), 410);
    }
}
