#[derive(Debug)]
pub struct Input {
    lines: Vec<Vec<u8>>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let lines = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        Self { lines }
    }
}

pub fn part1(input: &Input) -> usize {
    unimplemented!("part1")
}

pub fn part2(input: &Input) -> usize {
    unimplemented!("part2")
}
