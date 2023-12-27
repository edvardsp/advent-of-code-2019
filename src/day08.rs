// https://adventofcode.com/2019/day/8

use std::fmt;
use std::io::Write;

use termcolor::WriteColor;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA: usize = WIDTH * HEIGHT;

pub struct Input {
    image: String,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let image = value.to_string();
        Self { image }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Color {
    Black,
    White,
    Transparent,
}

impl Color {
    fn from(digit: u8) -> Self {
        match digit {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            _ => panic!("Invalid Color value: {}", digit),
        }
    }

    fn color_spec(&self) -> termcolor::ColorSpec {
        let mut cs = termcolor::ColorSpec::new();
        match self {
            Color::Black => cs.set_fg(None).set_bg(Some(termcolor::Color::Black)),
            Color::White => cs.set_fg(None).set_bg(Some(termcolor::Color::White)),
            Color::Transparent => cs.set_fg(None).set_bg(None),
        };
        cs
    }
}

pub struct Layer {
    colors: Vec<Color>,
}

impl Layer {
    fn new(n: usize) -> Self {
        Self {
            colors: [Color::Transparent].repeat(n),
        }
    }

    fn from(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    fn num_of(&self, color: Color) -> usize {
        self.colors
            .iter()
            .fold(0, |acc, c| if *c == color { acc + 1 } else { acc })
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = termcolor::Buffer::ansi();
        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                let color = self.colors.get(h * WIDTH + w).unwrap();
                buf.set_color(&color.color_spec()).unwrap();
                write!(buf, " ").unwrap();
            }
            writeln!(buf).unwrap();
        }
        writeln!(f)?;
        write!(f, "{}", String::from_utf8_lossy(buf.as_slice()))?;
        Ok(())
    }
}

pub fn part1(input: &Input) -> usize {
    let colors = parse_colors(&input.image);
    assert!(colors.len() % AREA == 0);

    let layer = colors
        .as_slice()
        .chunks_exact(AREA)
        .map(|c| Layer::from(c.to_vec()))
        .min_by(|lhs, rhs| lhs.num_of(Color::Black).cmp(&rhs.num_of(Color::Black)))
        .unwrap();

    layer.num_of(Color::White) * layer.num_of(Color::Transparent)
}

pub fn part2(input: &Input) -> Layer {
    let colors = parse_colors(&input.image);
    assert!(colors.len() % AREA == 0);

    let layers: Vec<Layer> = colors
        .as_slice()
        .chunks_exact(AREA)
        .map(|c| Layer::from(c.to_vec()))
        .collect();

    let mut decoded_layer = Layer::new(AREA);
    for layer in layers.iter() {
        let new_colors = decoded_layer
            .colors
            .iter()
            .zip(layer.colors.iter())
            .map(|(dc, c)| match (dc, c) {
                (Color::Transparent, new_color) => *new_color,
                (curr_color, _) => *curr_color,
            })
            .collect();

        decoded_layer = Layer::from(new_colors);
        if decoded_layer.num_of(Color::Transparent) == 0 {
            break;
        }
    }

    decoded_layer
}

fn parse_colors(input: &str) -> Vec<Color> {
    const RADIX: u32 = 10;

    input
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap() as _)
        .map(Color::from)
        .collect()
}
