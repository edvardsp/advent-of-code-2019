// https://adventofcode.com/2019/day/9

use core::panic;
use std::collections::VecDeque;
use std::str::FromStr;

type Integer = isize;

pub struct Input {
    tape: Tape,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let tape = Tape::from_str(value).unwrap();
        Self { tape }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl From<Integer> for ParamMode {
    fn from(value: Integer) -> Self {
        match value {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("invalid ParamMode value: {value}"),
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

impl From<Integer> for OpCode {
    fn from(value: Integer) -> Self {
        let param3: ParamMode = ((value / 10000) % 10).into();
        let param2: ParamMode = ((value / 1000) % 10).into();
        let param1: ParamMode = ((value / 100) % 10).into();
        let opcode = value % 100;
        match opcode {
            1 => OpCode::Add(param1, param2, param3),
            2 => OpCode::Mul(param1, param2, param3),
            3 => OpCode::Input(param1),
            4 => OpCode::Output(param1),
            5 => OpCode::JumpIfTrue(param1, param2),
            6 => OpCode::JumpIfFalse(param1, param2),
            7 => OpCode::LessThan(param1, param2, param3),
            8 => OpCode::Equals(param1, param2, param3),
            9 => OpCode::AdjustRelBase(param1),
            99 => OpCode::Eof,
            _ => panic!("invalid OpCode value: {value}"),
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

    fn get(&mut self, pos: Integer) -> Integer {
        assert!(pos >= 0);

        if pos >= self.mem.len() as isize {
            let new_len = (pos + 1) as usize;
            self.mem.resize(new_len, 0);
        }

        self.mem[pos as usize]
    }

    fn pget(&mut self, pos: Integer, param: ParamMode) -> Integer {
        let pos = match param {
            ParamMode::Position => self.get(pos),
            ParamMode::Immediate => pos,
            ParamMode::Relative => self.relbase + self.get(pos),
        };
        self.get(pos)
    }

    fn set(&mut self, pos: Integer, value: Integer) {
        assert!(pos >= 0);

        if pos >= self.mem.len() as isize {
            let new_len = (pos + 1) as usize;
            self.mem.resize(new_len, 0);
        }

        self.mem[pos as usize] = value;
    }

    fn run<I>(&mut self, mut input: I) -> RunStatus
    where
        I: Iterator<Item = Integer>,
    {
        if self.empty() {
            return RunStatus::Halt;
        }

        loop {
            let opcode: OpCode = self.get(self.pc).into();

            match opcode {
                OpCode::Add(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3),
                        ParamMode::Immediate => self.get(self.pc + 3),
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3),
                    };

                    let value = lhs + rhs;
                    self.set(dst, value);

                    self.pc += 4;
                }
                OpCode::Mul(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3),
                        ParamMode::Immediate => self.get(self.pc + 3),
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3),
                    };

                    let value = lhs * rhs;
                    self.set(dst, value);

                    self.pc += 4;
                }
                OpCode::Input(param1) => {
                    let dst = match param1 {
                        ParamMode::Position => self.get(self.pc + 1),
                        ParamMode::Immediate => self.get(self.pc + 1),
                        ParamMode::Relative => self.relbase + self.get(self.pc + 1),
                    };

                    match input.next() {
                        Some(value) => self.set(dst, value),
                        None => return RunStatus::Poll,
                    }

                    self.pc += 2;
                }
                OpCode::Output(param1) => {
                    let src = self.pget(self.pc + 1, param1);
                    self.output.push_back(src);

                    self.pc += 2;
                }
                OpCode::JumpIfTrue(param1, param2) => {
                    let cnd = self.pget(self.pc + 1, param1);
                    let val = self.pget(self.pc + 2, param2);

                    self.pc = if cnd != 0 { val } else { self.pc + 3 };
                }
                OpCode::JumpIfFalse(param1, param2) => {
                    let cnd = self.pget(self.pc + 1, param1);
                    let val = self.pget(self.pc + 2, param2);

                    self.pc = if cnd == 0 { val } else { self.pc + 3 };
                }
                OpCode::LessThan(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3),
                        ParamMode::Immediate => self.get(self.pc + 3),
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3),
                    };

                    let value = if lhs < rhs { 1 } else { 0 };
                    self.set(dst, value);

                    self.pc += 4;
                }
                OpCode::Equals(param1, param2, param3) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = match param3 {
                        ParamMode::Position => self.get(self.pc + 3),
                        ParamMode::Immediate => self.get(self.pc + 3),
                        ParamMode::Relative => self.relbase + self.get(self.pc + 3),
                    };

                    let value = if lhs == rhs { 1 } else { 0 };
                    self.set(dst, value);

                    self.pc += 4;
                }
                OpCode::AdjustRelBase(param1) => {
                    let adj = self.pget(self.pc + 1, param1);

                    self.relbase += adj;

                    self.pc += 2;
                }
                OpCode::Eof => return RunStatus::Halt,
            }
        }
    }
}

impl FromStr for Tape {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            mem: s.split(',').map(|i| i.parse()).collect::<Result<_, _>>()?,
            pc: 0,
            relbase: 0,
            output: VecDeque::new(),
        })
    }
}

pub fn part1(input: &Input) -> Integer {
    let mut tape = input.tape.clone();

    let run_status = tape.run([1].into_iter());
    assert_eq!(run_status, RunStatus::Halt);

    let output = tape.output();

    output[0]
}

pub fn part2(input: &Input) -> Integer {
    let mut tape = input.tape.clone();

    let run_status = tape.run([2].into_iter());
    assert_eq!(run_status, RunStatus::Halt);

    let output = tape.output();

    output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tape(tape_str: &str, input: Vec<Integer>) -> Vec<Integer> {
        let mut tape = Tape::from_str(tape_str).unwrap();
        let run_status = tape.run(input.into_iter());
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
