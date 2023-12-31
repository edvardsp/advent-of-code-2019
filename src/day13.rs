use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    tape: Tape,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let tape = Tape::from(value);
        Self { tape }
    }
}

type Integer = isize;

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

#[derive(Clone, Debug)]
struct Tape {
    mem: Vec<Integer>,
    pc: Integer,
    relbase: Integer,
}

impl From<&str> for Tape {
    fn from(value: &str) -> Self {
        let mem = value
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        Self {
            mem,
            pc: 0,
            relbase: 0,
        }
    }
}

enum Io {
    Input,
    Output(Integer),
}

impl Tape {
    fn empty(&self) -> bool {
        self.mem.is_empty()
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

    fn run<F>(&mut self, mut io: F) -> RunStatus
    where
        F: FnMut(Io) -> Option<Integer>,
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

                    match io(Io::Input) {
                        Some(value) => self.set(dst, value),
                        None => return RunStatus::Poll,
                    }

                    self.pc += 2;
                }
                OpCode::Output(param1) => {
                    let src = self.pget(self.pc + 1, param1);
                    io(Io::Output(src));

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

fn _map_to_string(shape: (isize, isize), map: &HashMap<(isize, isize), char>) -> String {
    let mut out = String::new();
    for y in 0..shape.0 {
        for x in 0..shape.1 {
            let coord = (y, x);
            out.push(map.get(&coord).copied().unwrap_or(' '));
        }
        out.push('\n');
    }
    out
}

pub fn part1(input: &Input) -> usize {
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut shape = (0, 0);
    let mut counter = 0;

    let mut tape = input.tape.clone();
    let status = tape.run(|io| {
        if let Io::Output(value) = io {
            let value = value as usize;
            counter += 1;
            if counter == 1 {
                pos.1 = value;
                shape.1 = shape.1.max(value + 1);
            } else if counter == 2 {
                pos.0 = value;
                shape.0 = shape.0.max(value + 1);
            } else {
                let c = match value {
                    0 => ' ',
                    1 => '|',
                    2 => '#',
                    3 => '-',
                    4 => '*',
                    _ => panic!("invalid tile value: {value}"),
                };
                map.insert(pos, c);
                counter = 0;
            }
        }
        None
    });

    assert_eq!(status, RunStatus::Halt);

    map.values().filter(|&&tile| tile == '#').count()
}

pub fn part2(input: &Input) -> usize {
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut shape = (0, 0);
    let mut counter = 0;
    let mut ball: (isize, isize) = (0, 0);
    let mut paddle: (isize, isize) = (0, 0);
    let mut score = 0;

    let mut tape = input.tape.clone();
    tape.set(0, 2);
    let status = tape.run(|io| {
        const SCORE: (isize, isize) = (0, -1);
        match io {
            Io::Input => {
                // println!("{}", _map_to_string(shape, &map));
                // std::thread::sleep(std::time::Duration::from_millis(16));
                let signum = (ball.1 - paddle.1).signum();
                Some(signum)
            }
            Io::Output(value) => {
                counter += 1;
                if counter == 1 {
                    pos.1 = value;
                    shape.1 = shape.1.max(value + 1);
                } else if counter == 2 {
                    pos.0 = value;
                    shape.0 = shape.0.max(value + 1);
                } else {
                    if pos == SCORE {
                        score = value as usize;
                    } else {
                        let c = match value {
                            0 => ' ',
                            1 => '|',
                            2 => '#',
                            3 => '-',
                            4 => '*',
                            _ => panic!("invalid tile value: {value}"),
                        };
                        if c == '*' {
                            ball = pos;
                        } else if c == '-' {
                            paddle = pos;
                        }
                        map.insert(pos, c);
                    }
                    counter = 0;
                }
                None
            }
        }
    });

    assert_eq!(status, RunStatus::Halt);

    score
}
