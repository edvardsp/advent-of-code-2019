use std::ops::AddAssign;

pub struct Input {
    moons: Vec<Moon>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let moons = value.lines().map(str::trim).map(Moon::from).collect();
        Self { moons }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn unsigned_abssum(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    fn signum(&self, other: &Self) -> Self {
        Self {
            x: (other.x - self.x).clamp(-1, 1),
            y: (other.y - self.y).clamp(-1, 1),
            z: (other.z - self.z).clamp(-1, 1),
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Copy, Clone, Debug)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    fn energy(&self) -> usize {
        self.pos.unsigned_abssum() * self.vel.unsigned_abssum()
    }
}

impl From<&str> for Moon {
    fn from(value: &str) -> Self {
        let value = value.strip_prefix('<').unwrap().strip_suffix('>').unwrap();
        let mut tokens = value.split(", ").map(|s| s[2..].parse().unwrap());
        let x = tokens.next().unwrap();
        let y = tokens.next().unwrap();
        let z = tokens.next().unwrap();
        let pos = Vector { x, y, z };
        let vel = Vector::default();
        Self { pos, vel }
    }
}

fn tick(moons: &mut [Moon]) {
    // Apply gravity
    for i in 0..moons.len() {
        for j in (0..moons.len()).filter(|&j| j != i) {
            moons[i].vel += moons[i].pos.signum(&moons[j].pos);
        }
    }

    // Apply velocity
    for moon in moons.iter_mut() {
        moon.pos += moon.vel;
    }
}

fn simulate(moons: &mut [Moon], n: usize) -> usize {
    for _tick in 1..=n {
        tick(moons);
    }
    moons.iter().map(Moon::energy).sum()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part1(input: &Input) -> usize {
    let mut moons = input.moons.clone();
    simulate(&mut moons, 1000)
}

pub fn part2(input: &Input) -> usize {
    let mut moons = input.moons.clone();
    let mut periods_x = None;
    let mut periods_y = None;
    let mut periods_z = None;

    for n in 1.. {
        tick(&mut moons);

        if periods_x.is_none()
            && (0..moons.len()).all(|i| {
                moons[i].pos.x == input.moons[i].pos.x && moons[i].vel.x == input.moons[i].vel.x
            })
        {
            periods_x = Some(n)
        }

        if periods_y.is_none()
            && (0..moons.len()).all(|i| {
                moons[i].pos.y == input.moons[i].pos.y && moons[i].vel.y == input.moons[i].vel.y
            })
        {
            periods_y = Some(n);
        }

        if periods_z.is_none()
            && (0..moons.len()).all(|i| {
                moons[i].pos.z == input.moons[i].pos.z && moons[i].vel.z == input.moons[i].vel.z
            })
        {
            periods_z = Some(n);
        }

        if let (Some(x), Some(y), Some(z)) = (periods_x, periods_y, periods_z) {
            return [x, y, z].into_iter().fold(1, lcm);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        const INPUT: &str = r#"<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"#;

        let mut input: Input = INPUT.into();
        assert_eq!(simulate(&mut input.moons, 10), 179);
    }

    #[test]
    fn test_part1_ex2() {
        const INPUT: &str = r#"<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>"#;

        let mut input: Input = INPUT.into();
        assert_eq!(simulate(&mut input.moons, 100), 1940);
    }

    #[test]
    fn test_part2_ex1() {
        const INPUT: &str = r#"<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"#;

        let input: Input = INPUT.into();
        assert_eq!(part2(&input), 2772);
    }

    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = r#"<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>"#;

        let input: Input = INPUT.into();
        assert_eq!(part2(&input), 4686774924);
    }
}
