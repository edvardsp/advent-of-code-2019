// https://adventofcode.com/2019/day/9

use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

type Integer = isize;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn new(value: Integer) -> Result<Self> {
        match value {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            2 => Ok(ParamMode::Relative),
            _ => Err(From::from(format!("Invalid param mode {}", value))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add(ParamMode, ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : dst = lhs + rhs
    Mul(ParamMode, ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : dst = lhs * rhs
    Input(ParamMode),                     // <op>,<dst>             : dst = *input*
    Output(ParamMode),                    // <op>,<src>             : *output* = src
    JumpIfTrue(ParamMode, ParamMode),     // <op>,<cnd>,<val>       : if cnd != 0 then pc = val
    JumpIfFalse(ParamMode, ParamMode),    // <op>,<cnd>,<val>       : if cnd == 0 then pc = val
    LessThan(ParamMode, ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : if lhs < rhs then dst = 1 else dst = 0
    Equals(ParamMode, ParamMode, ParamMode), // <op>,<lhs>,<rhs>,<dst> : if lhs == rhs then dst = 1 else dst = 0
    AdjustRelBase(ParamMode),                // <op>,<adj>              : relbase += adj
    Eof,
}

impl OpCode {
    fn new(value: Integer) -> Result<Self> {
        let param3 = ParamMode::new((value / 10000) % 10)?;
        let param2 = ParamMode::new((value / 1000) % 10)?;
        let param1 = ParamMode::new((value / 100) % 10)?;
        let opcode = value % 100;
        match opcode {
            1 => Ok(OpCode::Add(param1, param2, param3)),
            2 => Ok(OpCode::Mul(param1, param2, param3)),
            3 => Ok(OpCode::Input(param1)),
            4 => Ok(OpCode::Output(param1)),
            5 => Ok(OpCode::JumpIfTrue(param1, param2)),
            6 => Ok(OpCode::JumpIfFalse(param1, param2)),
            7 => Ok(OpCode::LessThan(param1, param2, param3)),
            8 => Ok(OpCode::Equals(param1, param2, param3)),
            9 => Ok(OpCode::AdjustRelBase(param1)),
            99 => Ok(OpCode::Eof),
            _ => Err(From::from(format!("Invalid opcode {}", value))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum RunStatus {
    Poll,
    Halt,
}

#[derive(Clone)]
struct Tape {
    mem: Vec<Integer>,
    pc: Integer,
    relbase: Integer,
    output: VecDeque<Integer>,
}

impl Tape {
    fn empty(&self) -> bool {
        self.mem.is_empty()
    }

    fn output(&mut self) -> Vec<Integer> {
        From::from(self.output.clone())
    }

    fn get(&mut self, pos: Integer) -> Result<Integer> {
        if pos < 0 {
            return Err(From::from(format!(
                "Out of bounds access to tape get: {}",
                pos
            )));
        }

        if pos >= self.mem.len() as isize {
            let new_len = (pos + 1) as usize;
            self.mem.resize(new_len, 0);
        }

        Ok(self.mem[pos as usize])
    }

    fn pget(&mut self, pos: Integer, param: ParamMode) -> Result<Integer> {
        match param {
            ParamMode::Position => {
                let pos = self.get(pos)?;
                Ok(self.get(pos)?)
            }
            ParamMode::Immediate => Ok(self.get(pos)?),
            ParamMode::Relative => {
                let pos = self.get(pos)?;
                Ok(self.get(self.relbase + pos)?)
            }
        }
    }

    fn set(&mut self, pos: Integer, value: Integer) -> Result<()> {
        if pos < 0 {
            return Err(From::from(format!(
                "Out of bounds access to tape get: {}",
                pos
            )));
        }

        if pos >= self.mem.len() as isize {
            let new_len = (pos + 1) as usize;
            self.mem.resize(new_len, 0);
        }

        self.mem[pos as usize] = value;
        Ok(())
    }

    fn run(&mut self, input: Vec<Integer>) -> Result<RunStatus> {
        if self.empty() {
            return Ok(RunStatus::Halt);
        }

        let mut input_iter = input.into_iter();

        loop {
            let opcode = OpCode::new(self.get(self.pc)?)?;

            match opcode {
                OpCode::Add(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3)?,
                        ParamMode::Immediate => self.get(self.pc + 3)?,
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3)?,
                    };

                    let result = lhs + rhs;
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Mul(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3)?,
                        ParamMode::Immediate => self.get(self.pc + 3)?,
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3)?,
                    };

                    let result = lhs * rhs;
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Input(param1) => {
                    let dst = match param1 {
                        ParamMode::Position => self.get(self.pc + 1)?,
                        ParamMode::Immediate => self.get(self.pc + 1)?,
                        ParamMode::Relative => self.relbase + self.get(self.pc + 1)?,
                    };

                    if let Some(result) = input_iter.next() {
                        self.set(dst, result)?;
                    } else {
                        return Ok(RunStatus::Poll);
                    }

                    self.pc += 2;
                }
                OpCode::Output(param1) => {
                    let src = self.pget(self.pc + 1, param1)?;
                    self.output.push_back(src);

                    self.pc += 2;
                }
                OpCode::JumpIfTrue(param1, param2) => {
                    let cnd = self.pget(self.pc + 1, param1)?;
                    let val = self.pget(self.pc + 2, param2)?;

                    self.pc = if cnd != 0 { val } else { self.pc + 3 };
                }
                OpCode::JumpIfFalse(param1, param2) => {
                    let cnd = self.pget(self.pc + 1, param1)?;
                    let val = self.pget(self.pc + 2, param2)?;

                    self.pc = if cnd == 0 { val } else { self.pc + 3 };
                }
                OpCode::LessThan(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3)?,
                        ParamMode::Immediate => self.get(self.pc + 3)?,
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3)?,
                    };

                    let result = if lhs < rhs { 1 } else { 0 };
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Equals(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3)?,
                        ParamMode::Immediate => self.get(self.pc + 3)?,
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3)?,
                    };

                    let result = if lhs == rhs { 1 } else { 0 };
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::AdjustRelBase(param1) => {
                    let adj = self.pget(self.pc + 1, param1)?;

                    self.relbase += adj;

                    self.pc += 2;
                }
                OpCode::Eof => return Ok(RunStatus::Halt),
            }
        }
    }
}

impl FromStr for Tape {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self {
            mem: s
                .split(',')
                .map(|i| i.parse())
                .collect::<::std::result::Result<_, _>>()?,
            pc: 0,
            relbase: 0,
            output: VecDeque::new(),
        })
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unexpected error reading from stdin");
    let input = input.trim();

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut tape: Tape = input.parse().unwrap();

    let run_status = tape.run(vec![1]).unwrap();
    assert_eq!(run_status, RunStatus::Halt);

    let output = tape.output();

    println!("part1: {:?}", output);
}

fn part2(input: &str) {
    let mut tape: Tape = input.parse().unwrap();

    let run_status = tape.run(vec![2]).unwrap();
    assert_eq!(run_status, RunStatus::Halt);

    let output = tape.output();

    println!("part2: {:?}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tape(tape_str: &str, input: Vec<Integer>) -> Vec<Integer> {
        let mut tape = Tape::from_str(tape_str).unwrap();
        let run_status = tape.run(input).unwrap();
        assert_eq!(run_status, RunStatus::Halt);
        tape.output()
    }

    #[test]
    fn test_part1_ex1() {
        assert_eq!(
            test_tape(
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                vec![]
            ),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(
            test_tape("1102,34915192,34915192,7,4,7,99,0", vec![]),
            vec![1219070632396864]
        );
    }

    #[test]
    fn test_part1_ex3() {
        assert_eq!(
            test_tape("104,1125899906842624,99", vec![]),
            vec![1125899906842624]
        );
    }
}
