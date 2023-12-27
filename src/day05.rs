// https://adventofcode.com/2019/day/2

#[derive(Debug)]
pub struct Input {
    tape: Tape,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let tape = value.into();
        Self { tape }
    }
}

type Integer = isize;

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
        let _param3 = ParamMode::from((value / 10000) % 10);
        let param2 = ParamMode::from((value / 1000) % 10);
        let param1 = ParamMode::from((value / 100) % 10);
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

#[derive(Clone, Debug)]
struct Tape {
    mem: Vec<Integer>,
}

impl From<&str> for Tape {
    fn from(value: &str) -> Self {
        let mem = value
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        Self { mem }
    }
}

impl Tape {
    fn get(&self, pos: usize) -> Integer {
        let mem = self.mem.get(pos).expect("invalid get position");
        *mem
    }

    fn pget(&self, pos: usize, param: ParamMode) -> Integer {
        let pos = match param {
            ParamMode::Position => self.get(pos) as usize,
            ParamMode::Immediate => pos,
        };
        self.get(pos)
    }

    fn set(&mut self, pos: usize, value: Integer) {
        let mem = self.mem.get_mut(pos).expect("invalid set pos");
        *mem = value;
    }
}

impl ToString for Tape {
    fn to_string(&self) -> String {
        let strings: Vec<String> = self.mem.iter().map(ToString::to_string).collect();
        strings.join(",")
    }
}

fn gravity_assist_program<I>(tape: &mut Tape, mut input: I) -> Vec<Integer>
where
    I: Iterator<Item = Integer>,
{
    let mut output = Vec::new();
    let mut pc = 0;

    loop {
        let opcode: OpCode = tape.get(pc).into();

        match opcode {
            OpCode::Add(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1);
                let rhs = tape.pget(pc + 2, param2);
                let dst = tape.get(pc + 3);

                let result = lhs + rhs;
                tape.set(dst as usize, result);

                pc += 4;
            }
            OpCode::Mul(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1);
                let rhs = tape.pget(pc + 2, param2);
                let dst = tape.get(pc + 3);

                let result = lhs * rhs;
                tape.set(dst as usize, result);

                pc += 4;
            }
            OpCode::Input => {
                let dst = tape.get(pc + 1);

                let result = input.next().unwrap();

                tape.set(dst as usize, result);

                pc += 2;
            }
            OpCode::Output(param1) => {
                let src = tape.pget(pc + 1, param1);

                output.push(src);

                pc += 2;
            }
            OpCode::JumpIfTrue(param1, param2) => {
                let cnd = tape.pget(pc + 1, param1);
                let val = tape.pget(pc + 2, param2);

                if cnd != 0 {
                    pc = val as usize;
                } else {
                    pc += 3;
                }
            }
            OpCode::JumpIfFalse(param1, param2) => {
                let cnd = tape.pget(pc + 1, param1);
                let val = tape.pget(pc + 2, param2);

                if cnd == 0 {
                    pc = val as usize;
                } else {
                    pc += 3;
                }
            }
            OpCode::LessThan(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1);
                let rhs = tape.pget(pc + 2, param2);
                let dst = tape.get(pc + 3);

                let result = if lhs < rhs { 1 } else { 0 };
                tape.set(dst as usize, result);

                pc += 4;
            }
            OpCode::Equals(param1, param2) => {
                let lhs = tape.pget(pc + 1, param1);
                let rhs = tape.pget(pc + 2, param2);
                let dst = tape.get(pc + 3);

                let result = if lhs == rhs { 1 } else { 0 };
                tape.set(dst as usize, result);

                pc += 4;
            }
            OpCode::Eof => break,
        }
    }

    output
}

pub fn part1(input: &Input) -> usize {
    let mut tape = input.tape.clone();

    let input = std::iter::repeat(1);

    let output = gravity_assist_program(&mut tape, input);
    output[0] as usize
}

pub fn part2(input: &Input) -> usize {
    let mut tape = input.tape.clone();

    let input = std::iter::repeat(5);

    let output = gravity_assist_program(&mut tape, input);
    output[0] as usize
}
