// https://adventofcode.com/2019/day/1

use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT: &'static str = "input/input.txt";

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

fn main() {
    let file = File::open(INPUT).expect("Unable to open file");

    let module_masses: Vec<usize> = BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("IO failed to read line")
                .parse::<usize>()
                .expect("Failed to parse number")
        })
        .collect();

    let total_fuel: usize = module_masses.iter().map(|n| calc_fuel(*n)).sum();
    println!("Total fuel: {}", total_fuel);

    let total_fuel_req: usize = module_masses.iter().map(|n| calc_fuel_req(*n)).sum();
    println!("Total fuel requirements: {}", total_fuel_req);
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
