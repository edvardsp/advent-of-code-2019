// https://adventofcode.com/2019/day/3

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Coord = (isize, isize);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(From::from("Invalid string format for Direction"));
        }
        let dir = match s.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => return Err(From::from(format!("Invalid Direction character {}", c))),
        };
        Ok(dir)
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

    fn path(&self, point: &Coord) -> Option<isize> {
        for (path, coord) in self.coords.iter().enumerate() {
            if coord == point {
                // Need to add one because the path starts at 1, while indices start at 0
                return Some(path as isize + 1);
            }
        }
        None
    }
}

impl FromStr for Wire {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let mut coords = Vec::new();
        let mut curr_coord = (0, 0);
        for segment in s.split(',') {
            let dir: Direction = segment[..1].parse()?;
            let len = segment[1..].parse()?;
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
        Ok(Self { coords, set })
    }
}

fn parse_wires() -> Result<(Wire, Wire)> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let wire1 = iterator
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))??
        .parse()?;
    let wire2 = iterator
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))??
        .parse()?;

    Ok((wire1, wire2))
}

fn main() {
    let (wire1, wire2) = parse_wires().expect("Error parsing wires from stdin");

    let man_dist = part1(&wire1, &wire2).expect("Error running part1");
    println!("Manhattant distance: {}", man_dist);

    let comb_path = part2(&wire1, &wire2).expect("Error running part2");
    println!("Shortest combined path: {}", comb_path);
}

fn manhattan_distance(coord: &Coord) -> isize {
    coord.0.abs() + coord.1.abs()
}

fn part1(wire1: &Wire, wire2: &Wire) -> Result<isize> {
    let result = wire1
        .intersection(&wire2)
        .into_iter()
        .filter(|coord| **coord != (0, 0))
        .map(manhattan_distance)
        .min()
        .unwrap();
    Ok(result)
}

fn part2(wire1: &Wire, wire2: &Wire) -> Result<isize> {
    let result = wire1
        .intersection(&wire2)
        .into_iter()
        .map(|i| wire1.path(i).unwrap() + wire2.path(i).unwrap())
        .min()
        .unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(
            part1(
                &"R8,U5,L5,D3".parse().unwrap(),
                &"U7,R6,D4,L4".parse().unwrap()
            )
            .unwrap(),
            6
        );
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(
            part1(
                &"R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap(),
                &"U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap()
            )
            .unwrap(),
            159
        );
    }

    #[test]
    fn test_part1_ex3() {
        assert_eq!(
            part1(
                &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
                    .parse()
                    .unwrap(),
                &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap()
            )
            .unwrap(),
            135
        );
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(
            part2(
                &"R8,U5,L5,D3".parse().unwrap(),
                &"U7,R6,D4,L4".parse().unwrap()
            )
            .unwrap(),
            30
        );
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(
            part2(
                &"R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap(),
                &"U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap()
            )
            .unwrap(),
            610
        );
    }

    #[test]
    fn test_part2_ex3() {
        assert_eq!(
            part2(
                &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
                    .parse()
                    .unwrap(),
                &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap()
            )
            .unwrap(),
            410
        );
    }
}
