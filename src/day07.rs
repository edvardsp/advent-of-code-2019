// https://adventofcode.com/2019/day/7

use std::collections::VecDeque;
use std::str::FromStr;

use itertools::Itertools;

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

#[derive(Copy, Clone, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

impl From<Integer> for ParamMode {
    fn from(value: Integer) -> Self {
        match value {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("invalid ParamMode value: {value}"),
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

impl From<Integer> for OpCode {
    fn from(value: Integer) -> Self {
        let _param3: ParamMode = ((value / 10000) % 10).into();
        let param2: ParamMode = ((value / 1000) % 10).into();
        let param1: ParamMode = ((value / 100) % 10).into();
        let opcode = value % 100;
        match opcode {
            1 => OpCode::Add(param1, param2),
            2 => OpCode::Mul(param1, param2),
            3 => OpCode::Input,
            4 => OpCode::Output(param1),
            5 => OpCode::JumpIfTrue(param1, param2),
            6 => OpCode::JumpIfFalse(param1, param2),
            7 => OpCode::LessThan(param1, param2),
            8 => OpCode::Equals(param1, param2),
            99 => OpCode::Eof,
            _ => panic!("invalid OpCode value: {value}"),
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

    fn halted(&self) -> bool {
        let opcode: OpCode = self.get(self.pc).into();
        opcode == OpCode::Eof
    }

    fn get(&self, pos: Integer) -> Integer {
        let mem = self.mem.get(pos as usize).expect("invalid get access");
        *mem
    }

    fn pget(&mut self, pos: Integer, param: ParamMode) -> Integer {
        let pos = match param {
            ParamMode::Position => self.get(pos),
            ParamMode::Immediate => pos,
        };
        self.get(pos)
    }

    fn set(&mut self, pos: Integer, value: Integer) {
        let mem = self.mem.get_mut(pos as usize).expect("invalid set access");
        *mem = value;
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
                OpCode::Add(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = self.get(self.pc + 3);

                    let result = lhs + rhs;
                    self.set(dst, result);

                    self.pc += 4;
                }
                OpCode::Mul(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = self.get(self.pc + 3);

                    let result = lhs * rhs;
                    self.set(dst, result);

                    self.pc += 4;
                }
                OpCode::Input => {
                    let dst = self.get(self.pc + 1);

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
                OpCode::LessThan(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = self.get(self.pc + 3);

                    let result = if lhs < rhs { 1 } else { 0 };
                    self.set(dst, result);

                    self.pc += 4;
                }
                OpCode::Equals(param1, param2) => {
                    let lhs = self.pget(self.pc + 1, param1);
                    let rhs = self.pget(self.pc + 2, param2);
                    let dst = self.get(self.pc + 3);

                    let result = if lhs == rhs { 1 } else { 0 };
                    self.set(dst, result);

                    self.pc += 4;
                }
                OpCode::Eof => return RunStatus::Halt,
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
                .collect::<::std::result::Result<_, _>>()
                .unwrap(),
            pc: 0,
            output: VecDeque::new(),
        })
    }
}

pub fn part1(input: &Input) -> Integer {
    find_max_signal(&input.tape)
}

pub fn part2(input: &Input) -> Integer {
    find_max_signal_feedback_loop(&input.tape)
}

fn find_max_signal(tape: &Tape) -> Integer {
    let mut max_thrust = 0;

    for settings in (0..5).permutations(5) {
        let signal = find_signal(tape.clone(), settings.as_slice());
        if signal > max_thrust {
            max_thrust = signal;
        }
    }

    max_thrust
}

fn find_max_signal_feedback_loop(tape: &Tape) -> Integer {
    let mut max_thrust = 0;

    for settings in (5..10).permutations(5) {
        let signal = find_signal_feedback_loop(tape.clone(), settings.as_slice());
        if signal > max_thrust {
            max_thrust = signal;
        }
    }

    max_thrust
}

fn find_signal(tape: Tape, settings: &[Integer]) -> Integer {
    let mut signal = 0;
    for setting in settings {
        let mut amp = tape.clone();
        amp.run([*setting, signal].into_iter());
        signal = amp
            .pop_output()
            .expect("Expected at least one output from tape");
    }
    signal
}

fn find_signal_feedback_loop(tape: Tape, settings: &[Integer]) -> Integer {
    let mut amps: Vec<(Integer, Tape)> = settings
        .iter()
        .map(|setting| (*setting, tape.clone()))
        .collect();
    for (setting, amp) in amps.iter_mut() {
        amp.run([*setting].into_iter());
    }

    let mut last_signal = 0;
    loop {
        let mut signal = last_signal;
        for (_, amp) in amps.iter_mut() {
            if amp.halted() {
                return last_signal;
            }
            amp.run([signal].into_iter());
            signal = amp
                .pop_output()
                .expect("Expected at least one output from tape");
        }
        last_signal = signal;
    }
}
