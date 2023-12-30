use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use ndarray::Array2;

#[derive(Debug)]
pub struct Input {
    map: Array2<char>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut lines = value.lines().peekable();
        let width = lines.peek().unwrap().len();
        let height = lines.count();

        let map = value.lines().flat_map(str::chars).collect();
        let map = Array2::from_shape_vec((height, width), map).unwrap();

        Self { map }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    while b != 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vector(isize, isize);

impl Vector {
    fn magnitude(&self) -> isize {
        gcd(self.1.unsigned_abs(), self.0.unsigned_abs()) as isize
    }

    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self(self.0 / magnitude, self.1 / magnitude)
    }

    fn angle(&self) -> f64 {
        use std::f64::consts::*;

        let rad = f64::atan2(self.0 as f64, self.1 as f64);
        if self.0.signum() < 0 && self.1.signum() < 0 {
            2.0 * PI + FRAC_PI_2 + rad
        } else {
            FRAC_PI_2 + rad
        }
    }

    fn score(&self) -> isize {
        self.0 + self.1 * 100
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.normalize()
            .angle()
            .total_cmp(&other.normalize().angle())
            .then_with(|| other.magnitude().cmp(&self.magnitude()))
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_location(asteroids: &[Vector]) -> (Vector, usize) {
    asteroids
        .iter()
        .map(|asteroid| {
            (
                *asteroid,
                asteroids
                    .iter()
                    .filter(|other| *other != asteroid)
                    .map(|other| (*other - *asteroid).normalize())
                    .collect::<HashSet<_>>()
                    .len(),
            )
        })
        .max_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .unwrap()
}

pub fn part1(input: &Input) -> usize {
    let asteroids: Vec<_> = input
        .map
        .indexed_iter()
        .filter_map(|(coord, c)| if *c == '#' { Some(coord) } else { None })
        .map(|(y, x)| Vector(y as isize, x as isize))
        .collect();

    find_location(&asteroids).1
}

pub fn part2(input: &Input) -> isize {
    let asteroids: Vec<_> = input
        .map
        .indexed_iter()
        .filter_map(|(coord, c)| if *c == '#' { Some(coord) } else { None })
        .map(|(y, x)| Vector(y as isize, x as isize))
        .collect();

    let station = find_location(&asteroids).0;

    let mut rotation: HashMap<Vector, Vec<Vector>> = HashMap::new();
    for asteorid in asteroids
        .into_iter()
        .filter(|asteroid| *asteroid != station)
        .map(|asteorid| asteorid - station)
    {
        rotation
            .entry(asteorid.normalize())
            .or_default()
            .push(asteorid);
    }

    for asteroids in rotation.values_mut() {
        asteroids.sort_by_key(|key| Reverse(key.magnitude()));
    }

    let mut angles: Vec<_> = rotation.keys().copied().collect();
    angles.sort_by(|lhs, rhs| lhs.angle().total_cmp(&rhs.angle()));

    let mut count = 0;
    for angle in angles.into_iter().cycle() {
        let asteroids = rotation.get_mut(&angle).unwrap();
        if let Some(asteorid) = asteroids.pop() {
            count += 1;
            if count == 200 {
                return (asteorid + station).score();
            }
        }
    }

    unimplemented!("part2")
}
