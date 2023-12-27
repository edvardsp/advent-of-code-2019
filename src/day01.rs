// https://adventofcode.com/2019/day/1

#[derive(Debug)]
pub struct Input {
    masses: Vec<usize>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let masses = value
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        Self { masses }
    }
}

fn calc_fuel(mass: usize) -> usize {
    if mass < 9 {
        0
    } else {
        mass / 3 - 2
    }
}

fn calc_fuel_req(mass: usize) -> usize {
    let mut mass = mass;
    let mut summa = 0;
    while mass > 0 {
        let fuel = calc_fuel(mass);
        summa += fuel;
        mass = fuel;
    }
    summa
}

pub fn part1(input: &Input) -> usize {
    input.masses.iter().copied().map(calc_fuel).sum()
}

pub fn part2(input: &Input) -> usize {
    input.masses.iter().copied().map(calc_fuel_req).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);
    }

    #[test]
    fn test_calc_fuel_req() {
        assert_eq!(calc_fuel_req(14), 2);
        assert_eq!(calc_fuel_req(1969), 966);
        assert_eq!(calc_fuel_req(100756), 50346);
    }
}
