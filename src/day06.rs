// https://adventofcode.com/2019/day/6

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    map: OrbitMap,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let map = OrbitMap::from_str(value).unwrap();
        Self { map }
    }
}

#[derive(Debug)]
struct Orbit {
    direct: RefCell<Option<String>>,
    num_orbits: Cell<Option<usize>>,
}

impl Orbit {
    fn new() -> Self {
        Self {
            direct: RefCell::new(None),
            num_orbits: Cell::new(None),
        }
    }

    fn direct(&self) -> Option<String> {
        self.direct.borrow().clone()
    }

    fn set_direct(&mut self, other: String) {
        self.direct.replace(Some(other));
    }

    fn num_orbits(&self) -> Option<usize> {
        self.num_orbits.get()
    }

    fn set_num_orbits(&self, num_orbits: usize) {
        self.num_orbits.set(Some(num_orbits));
    }
}

#[derive(Debug)]
struct OrbitMap {
    orbits: HashMap<String, Orbit>,
}

impl OrbitMap {
    fn num_orbits(&self, orbit_id: &str) -> usize {
        let orbit = self.orbits.get(orbit_id).unwrap();
        if let Some(num) = orbit.num_orbits() {
            num
        } else {
            let num = if let Some(direct) = orbit.direct() {
                self.num_orbits(&direct) + 1
            } else {
                0
            };
            orbit.set_num_orbits(num);
            num
        }
    }

    fn total_orbits(&self) -> usize {
        let mut summa = 0;
        for orbit_id in self.orbits.keys() {
            summa += self.num_orbits(orbit_id);
        }
        summa
    }

    fn traverse(&self, orbit1: &str, orbit2: &str) -> usize {
        let num_o1 = self.num_orbits(orbit1);
        let num_o2 = self.num_orbits(orbit2);

        let (s, e) = if num_o1 < num_o2 {
            (orbit1, orbit2)
        } else {
            (orbit2, orbit1)
        };

        let mut paths: Vec<String> = Vec::new();

        let mut orbit = self.orbits.get(s).unwrap();
        paths.push(s.to_owned());
        while let Some(direct) = orbit.direct() {
            orbit = self.orbits.get(&direct).unwrap();
            paths.push(direct);
        }

        let mut orbit = self.orbits.get(e).unwrap();
        while let Some(direct) = orbit.direct() {
            orbit = self.orbits.get(&direct).unwrap();
            if paths.contains(&direct) {
                break;
            }
        }

        let num_intersect = orbit.num_orbits().unwrap();
        (num_o1 - num_intersect - 1) + (num_o2 - num_intersect - 1)
    }
}

impl FromStr for OrbitMap {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let mut orbits = HashMap::new();
        for line in s.lines() {
            let mut iter = line.splitn(2, ')').map(|o| o.to_string());

            let (o1_id, o2_id) = (iter.next().unwrap(), iter.next().unwrap());
            let mut o2 = Orbit::new();
            o2.set_direct(o1_id.clone());

            orbits.entry(o1_id).or_insert_with(Orbit::new);
            orbits.insert(o2_id, o2);
        }
        Ok(Self { orbits })
    }
}

pub fn part1(input: &Input) -> usize {
    input.map.total_orbits()
}

pub fn part2(input: &Input) -> usize {
    input.map.traverse("YOU", "SAN")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(part1(&INPUT.into()), 42);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(part2(&INPUT.into()), 4);
    }
}
