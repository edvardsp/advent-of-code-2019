use std::fs::File;
use std::io::Read;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("logic error: {0}")]
    Logic(String),
}

pub fn parse_input<I>(day: &str) -> Result<I, Error>
where
    for<'s> I: Sized + From<&'s str>,
{
    let filename = format!("input/{}.txt", day);
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content.as_str().into())
}

macro_rules! aoc_impl {
    ($($day:ident),*) => {
        $(mod $day;)*

        fn run_day(day: usize) {
            assert!((1..=25).contains(&day));
            match format!("day{:02}", day).as_ref() {
                $(
                    stringify!($day) => {
                        println!(stringify!($day));
                        let input: $day::Input = parse_input(stringify!($day)).expect("Failed to parse input");
                        println!(">> part1: {}", $day::part1(&input));
                        println!(">> part2: {}", $day::part2(&input));
                    }
                )*
                _ => unreachable!(),
            }
        }

        fn run(day: Option<usize>) {
            if let Some(day) = day {
                run_day(day);
            } else {
                for day in 1..=25 {
                    run_day(day);
                }
            }
        }
    }
}

aoc_impl!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14
);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map(|n| n.parse().unwrap());
    run(day);
}
