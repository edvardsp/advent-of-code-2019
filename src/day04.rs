// https://adventofcode.com/2019/day/4

use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Input {
    range: RangeInclusive<usize>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (from, to) = value.split_once('-').unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        let range = from..=to;
        Self { range }
    }
}

#[derive(Clone, Copy)]
struct Password(usize);

impl Iterator for Password {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let digit = self.0 % 10;
            self.0 /= 10;
            Some(digit)
        } else {
            None
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let range = input.range.clone();
    range.map(Password).filter(validate1).count()
}

pub fn part2(input: &Input) -> usize {
    let range = input.range.clone();
    range.map(Password).filter(validate2).count()
}

fn validate1(password: &Password) -> bool {
    let mut adj_digits = false;

    let ascending = password
        .zip(password.skip(1))
        .scan(true, |state, (d0, d1)| {
            if !adj_digits {
                adj_digits = d0 == d1;
            }
            if *state {
                *state = d0 >= d1;
            }
            Some(*state)
        })
        .all(|x| x);

    adj_digits && ascending
}

#[derive(PartialEq)]
enum AdjacentDigits {
    Ok,
    Maybe(usize),
    NotOk(usize),
    None,
}

fn validate2(password: &Password) -> bool {
    let mut adj_digits = AdjacentDigits::None;

    let ascending = password
        .zip(password.skip(1))
        .scan(true, |state, (d0, d1)| {
            match adj_digits {
                // Adjacent digits has been found and verified, no longer needed to verify.
                AdjacentDigits::Ok => {}
                // Adjacent digits has been found, but hasn't been verified.
                AdjacentDigits::Maybe(d) => {
                    adj_digits = if d == d1 {
                        AdjacentDigits::NotOk(d)
                    } else {
                        AdjacentDigits::Ok
                    };
                }
                // Adjacent digits has been found as part of a larger group, make sure we only
                // Start searching again until we find a new digit.
                AdjacentDigits::NotOk(d) => {
                    if d != d1 {
                        adj_digits = AdjacentDigits::None;
                    }
                }
                // No adjacent digits has been found
                AdjacentDigits::None => {
                    if d0 == d1 {
                        adj_digits = AdjacentDigits::Maybe(d1);
                    }
                }
            }

            if *state {
                *state = d0 >= d1;
            }
            Some(*state)
        })
        .all(|x| x);

    if let AdjacentDigits::Maybe(_) = adj_digits {
        adj_digits = AdjacentDigits::Ok;
    }

    adj_digits == AdjacentDigits::Ok && ascending
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(validate1(&Password(111111)));
        assert!(!validate1(&Password(223450)));
        assert!(!validate1(&Password(123789)));
    }

    #[test]
    fn test_part2_ex1() {
        assert!(validate2(&Password(112233)));
        assert!(!validate2(&Password(123444)));
        assert!(!validate2(&Password(589999)));
    }
}
