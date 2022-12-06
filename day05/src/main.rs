#![feature(iterator_try_collect)]
#![feature(iter_array_chunks)]

use std::{cmp::max, io::stdin};

#[derive(Debug, Default)]
enum ParseState {
    #[default]
    Placement,
    Instructions,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
enum CrateMoverModel {
    M9000,
    #[default]
    M9001,
}

#[derive(Default, Debug)]
struct State {
    parse_state: ParseState,
    stacks: Vec<Vec<Crate>>,
    model: CrateMoverModel,
}

#[derive(Debug, Clone)]
struct Crate {
    name: String,
}

impl Crate {
    fn empty() -> Crate {
        Crate {
            name: " ".to_string(),
        }
    }
}

impl TryFrom<char> for Crate {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_whitespace() {
            Err(())
        } else {
            Ok(Crate { name: value.into() })
        }
    }
}

impl State {
    fn add_crate(&mut self, stack: usize, cr: Option<Crate>) {
        self.stacks
            .resize(max(stack + 1, self.stacks.len()), vec![]);
        if let Some(cr) = cr {
            self.stacks[stack].insert(0, cr);
        }
    }

    fn mv(&mut self, count: usize, src: usize, dst: usize) -> Option<String> {
        let src_stack = self.stacks.get_mut(src - 1)?;
        let mut moves = match self.model {
            CrateMoverModel::M9000 => src_stack
                .split_off(src_stack.len() - count)
                .into_iter()
                .rev()
                .collect(),
            CrateMoverModel::M9001 => src_stack.split_off(src_stack.len() - count),
        };
        let dst_stack = self.stacks.get_mut(dst - 1)?;
        dst_stack.append(&mut moves);
        Some(self.answer())
    }

    fn answer(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&Crate::empty()).name.clone())
            .collect()
    }

    fn parse_line(&mut self, line: &str) -> Result<String, String> {
        if line.is_empty() {
            return Ok(self.answer());
        }
        match self.parse_state {
            ParseState::Placement => {
                // the line indicating that the input file is done specifying initial configuration
                if line.trim_start().starts_with("1") {
                    self.parse_state = ParseState::Instructions;
                    return Ok(self.answer());
                }
                for (stack, item) in line
                    .chars()
                    .chain([' '])
                    .array_chunks::<4>()
                    .map(|x| x[1])
                    .enumerate()
                {
                    self.add_crate(stack, item.try_into().ok());
                }
                Ok(self.answer())
            }
            ParseState::Instructions => {
                if let Some([count, src, dst]) = line
                    .split_whitespace()
                    .skip(1)
                    .step_by(2)
                    .map(|s| s.parse::<usize>())
                    .try_collect::<Vec<_>>()
                    .map_err(|err| err.to_string())?
                    .get(0..3)
                {
                    Ok(self
                        .mv(*count, *src, *dst)
                        .ok_or(format!("operation error ({}, {}, {})", count, src, dst))?)
                } else {
                    Err(format!("parsing error for instruction {}", line))
                }
            }
        }
    }
}

fn main() {
    let mut state = State::default();

    // Collect a vector of answers, each entry from each processed line.
    let answer = stdin()
        .lines()
        .map(|line| {
            line.map_err(|err| err.to_string())
                .and_then(|line| state.parse_line(&line))
        })
        .collect::<Result<Vec<String>, String>>()
        .unwrap();

    println!("{}", answer.last().unwrap());
}
