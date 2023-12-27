// https://adventofcode.com/2019/day/2

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

#[derive(PartialEq)]
enum OpCode {
    Add,
    Mul,
    Eof,
}

impl From<usize> for OpCode {
    fn from(value: usize) -> Self {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Eof,
            _ => panic!("invalid opcode: {}", value),
        }
    }
}

#[derive(Clone, Debug)]
struct Tape {
    mem: Vec<usize>,
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
    fn get(&self, pos: usize) -> usize {
        let mem = self.mem.get(pos).expect("invalid get access to tape");
        *mem
    }

    fn set(&mut self, pos: usize, value: usize) {
        let mem = self.mem.get_mut(pos).expect("invalid set access to tape");
        *mem = value;
    }
}

impl ToString for Tape {
    fn to_string(&self) -> String {
        let strings: Vec<String> = self.mem.iter().map(ToString::to_string).collect();
        strings.join(",")
    }
}

fn gravity_assist_program(tape: &mut Tape) {
    let mut pc = 0;

    loop {
        let opcode: OpCode = tape.get(pc).into();

        match opcode {
            OpCode::Add => {
                let lhs_addr = tape.get(pc + 1);
                let lhs = tape.get(lhs_addr);
                let rhs_addr = tape.get(pc + 2);
                let rhs = tape.get(rhs_addr);
                let dst = tape.get(pc + 3);

                let result = lhs + rhs;
                tape.set(dst, result);

                pc += 4;
            }
            OpCode::Mul => {
                let lhs_addr = tape.get(pc + 1);
                let lhs = tape.get(lhs_addr);
                let rhs_addr = tape.get(pc + 2);
                let rhs = tape.get(rhs_addr);
                let dst = tape.get(pc + 3);

                let result = lhs * rhs;
                tape.set(dst, result);

                pc += 4;
            }
            OpCode::Eof => break,
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let mut tape = input.tape.clone();

    tape.set(1, 12);
    tape.set(2, 2);

    gravity_assist_program(&mut tape);

    tape.get(0)
}

pub fn part2(input: &Input) -> usize {
    const TARGET: usize = 19_690_720;

    (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .find_map(|(noun, verb)| {
            let mut tape = input.tape.clone();

            tape.set(1, noun);
            tape.set(2, verb);

            gravity_assist_program(&mut tape);

            let output = tape.get(0);
            if output == TARGET {
                Some(100 * noun + verb)
            } else {
                None
            }
        })
        .expect("unable to find target")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tape<I: Into<Input>>(value: I) -> String {
        let mut input: Input = value.into();
        gravity_assist_program(&mut input.tape);
        input.tape.to_string()
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_tape("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(run_tape("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(run_tape("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(run_tape("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}
