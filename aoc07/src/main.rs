// https://adventofcode.com/2019/day/7

extern crate itertools;

use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

use itertools::Itertools;

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

enum RunStatus {
    Poll,
    Halt,
}

#[derive(Clone)]
struct Tape {
    mem: Vec<Integer>,
    pc: isize,
    output: VecDeque<Integer>,
}

impl Tape {
    fn empty(&self) -> bool {
        self.mem.is_empty()
    }

    fn pop_output(&mut self) -> Option<Integer> {
        self.output.pop_front()
    }

    fn halted(&self) -> Result<bool> {
        Ok(OpCode::new(self.get(self.pc)?)? == OpCode::Eof)
    }

    fn get(&self, pos: Integer) -> Result<Integer> {
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

    fn run(&mut self, input: Vec<Integer>) -> Result<RunStatus> {
        if self.empty() {
            return Ok(RunStatus::Halt);
        }

        let mut input_iter = input.into_iter();

        loop {
            let opcode = OpCode::new(self.get(self.pc)?)?;

            match opcode {
                OpCode::Add(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = self.get(self.pc + 3)?;

                    let result = lhs + rhs;
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Mul(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = self.get(self.pc + 3)?;

                    let result = lhs * rhs;
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Input => {
                    let dst = self.get(self.pc + 1)?;

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
                OpCode::LessThan(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = self.get(self.pc + 3)?;

                    let result = if lhs < rhs { 1 } else { 0 };
                    self.set(dst, result)?;

                    self.pc += 4;
                }
                OpCode::Equals(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1)?;
                    let rhs = self.pget(self.pc + 2, param2)?;
                    let dst = self.get(self.pc + 3)?;

                    let result = if lhs == rhs { 1 } else { 0 };
                    self.set(dst, result)?;

                    self.pc += 4;
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
    let original_tape: Tape = input.parse().expect("Unable to parse input for Tape");

    let output = find_max_signal(original_tape).expect("Unexpected error running part1");
    println!("part1: {}", output);
}

fn part2(input: &str) {
    let original_tape: Tape = input.parse().expect("Unable to parse input for Tape");

    let output =
        find_max_signal_feedback_loop(original_tape).expect("Unexpected error running part1");
    println!("part2: {}", output);
}

fn find_max_signal(tape: Tape) -> Result<Integer> {
    let mut max_thrust = 0;

    for settings in (0..5).permutations(5) {
        let signal = find_signal(tape.clone(), settings.as_slice())?;
        if signal > max_thrust {
            max_thrust = signal;
        }
    }

    Ok(max_thrust)
}

fn find_max_signal_feedback_loop(tape: Tape) -> Result<Integer> {
    let mut max_thrust = 0;

    for settings in (5..10).permutations(5) {
        let signal = find_signal_feedback_loop(tape.clone(), settings.as_slice())?;
        if signal > max_thrust {
            max_thrust = signal;
        }
    }

    Ok(max_thrust)
}

fn find_signal(tape: Tape, settings: &[Integer]) -> Result<Integer> {
    let mut signal = 0;
    for setting in settings {
        let mut amp = tape.clone();
        amp.run(vec![*setting, signal])?;
        signal = amp
            .pop_output()
            .expect("Expected at least one output from tape");
    }
    Ok(signal)
}

fn find_signal_feedback_loop(tape: Tape, settings: &[Integer]) -> Result<Integer> {
    let mut amps: Vec<(Integer, Tape)> = settings
        .iter()
        .map(|setting| (*setting, tape.clone()))
        .collect();
    for (setting, amp) in amps.iter_mut() {
        amp.run(vec![*setting])?;
    }

    let mut last_signal = 0;
    loop {
        let mut signal = last_signal;
        for (_, amp) in amps.iter_mut() {
            if amp.halted()? {
                return Ok(last_signal);
            }
            amp.run(vec![signal])?;
            signal = amp
                .pop_output()
                .expect("Expected at least one output from tape");
        }
        last_signal = signal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(
            find_max_signal(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
                    .parse()
                    .unwrap()
            )
            .unwrap(),
            43210
        );
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(
            find_max_signal(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
                    .parse()
                    .unwrap()
            )
            .unwrap(),
            54321
        );
    }

    #[test]
    fn test_part1_ex3() {
        assert_eq!(
            find_max_signal(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
                    .parse()
                    .unwrap()
                )
                .unwrap(),
                65210
            );
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(
            find_max_signal_feedback_loop(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .parse()
                .unwrap()
            )
            .unwrap(),
            139629729
        );
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(
            find_max_signal_feedback_loop(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                .parse()
                .unwrap()
            )
            .unwrap(),
            18216
        );
    }
}
