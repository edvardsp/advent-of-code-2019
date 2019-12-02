// https://adventofcode.com/2019/day/2

use std::io;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(PartialEq)]
enum OpCode {
    Add,
    Mul,
    Eof,
}

impl OpCode {
    fn new(value: usize) -> Result<Self> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            99 => Ok(OpCode::Eof),
            _ => Err(From::from(format!("Found invalid opcode {}", value))),
        }
    }
}

struct Tape {
    mem: Vec<usize>,
}

impl Tape {
    fn new(input: &str) -> Result<Self> {
        Ok(Self {
            mem: input
                .split(',')
                .map(|i| i.parse())
                .collect::<::std::result::Result<Vec<usize>, _>>()?,
        })
    }

    fn empty(&self) -> bool {
        self.mem.is_empty()
    }

    fn get(&self, pos: usize) -> Result<usize> {
        if pos < self.mem.len() {
            Ok(self.mem[pos])
        } else {
            Err(From::from(format!(
                "Invalid get access to tape, pos: {}, len: {}",
                pos,
                self.mem.len()
            )))
        }
    }

    fn set(&mut self, pos: usize, value: usize) -> Result<()> {
        if pos < self.mem.len() {
            self.mem[pos] = value;
            Ok(())
        } else {
            Err(From::from(format!(
                "Invalid set access to tape, pos: {}, len: {}",
                pos,
                self.mem.len()
            )))
        }
    }
}

impl ToString for Tape {
    fn to_string(&self) -> String {
        let strings: Vec<String> = self.mem.iter().map(ToString::to_string).collect();
        strings.join(",")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unexpected error reading from stdin");
    let input = input.trim();

    let part1_result = part1(&input).expect("Unexpected error running part1");
    println!("part1 result: {}", part1_result);

    let part2_result = part2(&input).expect("Unexpected error running part2");
    println!("part2 result: {}", part2_result);
}

fn gravity_assist_program(tape: &mut Tape) -> Result<()> {
    if tape.empty() {
        return Ok(());
    }

    let mut pc = 0;

    loop {
        let opcode = OpCode::new(tape.get(pc)?)?;

        match opcode {
            OpCode::Add => {
                let lhs_addr = tape.get(pc + 1)?;
                let lhs = tape.get(lhs_addr)?;
                let rhs_addr = tape.get(pc + 2)?;
                let rhs = tape.get(rhs_addr)?;
                let dst = tape.get(pc + 3)?;

                let result = lhs + rhs;
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Mul => {
                let lhs_addr = tape.get(pc + 1)?;
                let lhs = tape.get(lhs_addr)?;
                let rhs_addr = tape.get(pc + 2)?;
                let rhs = tape.get(rhs_addr)?;
                let dst = tape.get(pc + 3)?;

                let result = lhs * rhs;
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Eof => break,
        }
    }

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut tape = Tape::new(input)?;

    tape.set(1, 12)?;
    tape.set(2, 2)?;

    gravity_assist_program(&mut tape)?;

    Ok(tape.get(0)?)
}

fn part2(input: &str) -> Result<usize> {
    const TARGET: usize = 19_690_720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut tape = Tape::new(input)?;

            tape.set(1, noun)?;
            tape.set(2, verb)?;

            gravity_assist_program(&mut tape)?;

            let output = tape.get(0)?;
            if output == TARGET {
                println!("Success: noun = {}, verb = {}", noun, verb);
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(From::from(
        "Unable to find noun and verb combination for part2",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tape(input: &str) -> Result<String> {
        let mut tape = Tape::new(input)?;
        gravity_assist_program(&mut tape)?;
        Ok(tape.to_string())
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_tape("1,0,0,0,99").unwrap(), "2,0,0,0,99".to_owned());
        assert_eq!(run_tape("2,3,0,3,99").unwrap(), "2,3,0,6,99".to_owned());
        assert_eq!(
            run_tape("2,4,4,5,99,0").unwrap(),
            "2,4,4,5,99,9801".to_owned()
        );
        assert_eq!(
            run_tape("1,1,1,4,99,5,6,0,99").unwrap(),
            "30,1,1,4,2,5,6,0,99".to_owned()
        );
    }
}
