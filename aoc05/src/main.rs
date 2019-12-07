// https://adventofcode.com/2019/day/2

use std::include_str;
use std::io::{self, Write};

const INTCODE_PROGRAM: &'static str = include_str!("../input/input.txt");

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Integer = isize;

#[derive(Copy, Clone, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn new(value: Integer) -> Result<Self> {
        match value {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            _ => Err(From::from(format!("Invalid param mode {}", value))),
        }
    }
}

#[derive(PartialEq)]
enum OpCode {
    Add(ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : dst = lhs + rhs
    Mul(ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : dst = lhs * rhs
    Input,                     // <op>,<dst>             : dst = *input*
    Output(ParamMode),         // <op>,<src>             : *output* = src
    JumpIfTrue(ParamMode, ParamMode), // <op>,<cnd>,<val>       : if cnd != 0 then pc = val
    JumpIfFalse(ParamMode, ParamMode), // <op>,<cnd>,<val>       : if cnd == 0 then pc = val
    LessThan(ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : if lhs < rhs then dst = 1 else dst = 0
    Equals(ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : if lhs == rhs then dst = 1 else dst = 0
    Eof,
}

impl OpCode {
    fn new(value: Integer) -> Result<Self> {
        let _param3 = ParamMode::new((value / 10000) % 10)?;
        let param2 = ParamMode::new((value / 1000) % 10)?;
        let param1 = ParamMode::new((value / 100) % 10)?;
        let opcode = value % 100;
        match opcode {
            1 => Ok(OpCode::Add(param1, param2)),
            2 => Ok(OpCode::Mul(param1, param2)),
            3 => Ok(OpCode::Input),
            4 => Ok(OpCode::Output(param1)),
            5 => Ok(OpCode::JumpIfTrue(param1, param2)),
            6 => Ok(OpCode::JumpIfFalse(param1, param2)),
            7 => Ok(OpCode::LessThan(param1, param2)),
            8 => Ok(OpCode::Equals(param1, param2)),
            99 => Ok(OpCode::Eof),
            _ => Err(From::from(format!("Invalid opcode {}", value))),
        }
    }
}

struct Tape {
    mem: Vec<Integer>,
}

impl Tape {
    fn new(input: &str) -> Result<Self> {
        Ok(Self {
            mem: input
                .split(',')
                .map(|i| i.parse())
                .collect::<::std::result::Result<_, _>>()?,
        })
    }

    fn empty(&self) -> bool {
        self.mem.is_empty()
    }

    fn get(&mut self, pos: Integer) -> Result<Integer> {
        if 0 <= pos && pos < self.mem.len() as isize {
            Ok(self.mem[pos as usize])
        } else {
            Err(From::from(format!(
                "Out of bounds access to tape get: {}",
                pos
            )))
        }
    }

    fn pget(&mut self, pos: Integer, param: ParamMode) -> Result<Integer> {
        match param {
            ParamMode::Position => {
                let pos = self.get(pos)?;
                Ok(self.get(pos)?)
            }
            ParamMode::Immediate => Ok(self.get(pos)?),
        }
    }

    fn set(&mut self, pos: Integer, value: Integer) -> Result<()> {
        if pos >= 0 {
            self.mem[pos as usize] = value;
            Ok(())
        } else {
            Err(From::from(format!(
                "Out of bounds access to tape set, pos = {}",
                pos
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
    let input = INTCODE_PROGRAM;

    part(input).expect("Unexpected error running part1/2");
}

fn gravity_assist_program(tape: &mut Tape) -> Result<()> {
    if tape.empty() {
        return Ok(());
    }

    let input = io::stdin();
    let mut output = io::stdout();

    let mut pc = 0;

    loop {
        let opcode = OpCode::new(tape.get(pc)?)?;

        match opcode {
            OpCode::Add(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1)?;
                let rhs = tape.pget(pc + 2, param2)?;
                let dst = tape.get(pc + 3)?;

                let result = lhs + rhs;
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Mul(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1)?;
                let rhs = tape.pget(pc + 2, param2)?;
                let dst = tape.get(pc + 3)?;

                let result = lhs * rhs;
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Input => {
                let dst = tape.get(pc + 1)?;

                let mut input_str = String::new();
                input.read_line(&mut input_str)?;
                let len = input_str.trim_end_matches(&['\r', '\n'][..]).len();
                input_str.truncate(len);
                let result = input_str.parse()?;

                tape.set(dst, result)?;

                pc += 2;
            }
            OpCode::Output(param1) => {
                let src = tape.pget(pc + 1, param1)?;
                let result: String = format!("{}\n", src);

                output.write(result.as_bytes())?;
                output.flush()?;

                pc += 2;
            }
            OpCode::JumpIfTrue(param1, param2) => {
                let cnd = tape.pget(pc + 1, param1)?;
                let val = tape.pget(pc + 2, param2)?;

                if cnd != 0 {
                    pc = val;
                } else {
                    pc += 3;
                }
            }
            OpCode::JumpIfFalse(param1, param2) => {
                let cnd = tape.pget(pc + 1, param1)?;
                let val = tape.pget(pc + 2, param2)?;

                if cnd == 0 {
                    pc = val;
                } else {
                    pc += 3;
                }
            }
            OpCode::LessThan(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1)?;
                let rhs = tape.pget(pc + 2, param2)?;
                let dst = tape.get(pc + 3)?;

                let result = if lhs < rhs { 1 } else { 0 };
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Equals(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1)?;
                let rhs = tape.pget(pc + 2, param2)?;
                let dst = tape.get(pc + 3)?;

                let result = if lhs == rhs { 1 } else { 0 };
                tape.set(dst, result)?;

                pc += 4;
            }
            OpCode::Eof => break,
        }
    }

    Ok(())
}

fn part(input: &str) -> Result<()> {
    let mut tape = Tape::new(input)?;

    gravity_assist_program(&mut tape)?;

    Ok(())
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
