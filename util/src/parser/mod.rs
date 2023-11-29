use itertools::Itertools;
use std::collections::VecDeque;
mod equation;
use crate::util::get_first_match;
use regex::Regex;

pub struct Param {
    /// Determines the number of lines in a chunk
    pub block_size: usize,
    /// Details the regex pattern for each line
    pub match_types: Vec<Target>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum EqType {
    A,
    B,
}

/// Specifies what pattern is to be extracted from a given string
#[derive(Clone)]
pub enum Target {
    /// A number
    Number,
    /// A collection of comma separated values
    CSV(DataStruct),
    /// A mathematical equation
    Equation,
}

#[derive(Clone)]
pub enum DataStruct {
    /// In a Vector
    Vec,
    /// In a double ended queu
    VecDeque,
}

pub struct Closure(Box<dyn Fn(u64) -> u64>);
impl Copy for Closure {}
impl Clone for Closure {
    fn clone(&self) -> Self {
        *self
    }
}
/// The different types that the parser might return
pub enum Match {
    /// An i32
    Number(i32),
    /// A vector of i32
    Vec(Vec<i32>),
    /// A Double ended queu of i32
    VecDeque(VecDeque<i32>),
    /// A simple closure that takes an i32 as an input and applies some operation to it
    Closure(Closure),
}

impl Match {
    pub fn as_number(&self) -> Option<i32> {
        match self {
            Match::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<Vec<i32>> {
        match self {
            Match::Vec(vec) => Some(vec.clone()),
            _ => None,
        }
    }

    pub fn as_vecdeque(&self) -> Option<VecDeque<i32>> {
        match self {
            Match::VecDeque(deque) => Some(deque.clone()),
            _ => None,
        }
    }

    pub fn as_closure(self) -> Box<dyn Fn(u64) -> u64> {
        match self {
            Match::Closure(cs) => cs.0,
            _ => Box::new(move |x| x + 0),
        }
    }
}

// todo - Optimise this to iterate once
pub fn parse(content: &String, p: &Param) -> Vec<Vec<Match>> {
    let blocks: Vec<Vec<&str>> = content
        .lines()
        .chunks(p.block_size)
        .into_iter()
        .map(|line| line.collect::<Vec<_>>())
        .collect();

    let mut block_matches = vec![];
    for block in blocks {
        let mut line_matches = vec![];
        for (idx, target) in p.match_types.clone().into_iter().enumerate() {
            match parse_line(block[idx], target) {
                Some(m) => line_matches.push(m),
                None => {}
            }
        }
        block_matches.push(line_matches);
    }

    block_matches
}

fn parse_line(line: &str, t: Target) -> Option<Match> {
    match t {
        Target::Number => {
            let number: i32 = get_first_match(line, r"-?\d+")?.parse::<i32>().ok()?;
            Some(Match::Number(number))
        }
        Target::CSV(return_struct) => {
            let csv = get_first_match(line, r"(-?\d+)(,[[:blank:]]?-?\d+)*")?
                .split(',')
                .map(|s| s.trim())
                .filter_map(|s| s.parse::<i32>().ok());
            match return_struct {
                DataStruct::Vec => Some(Match::Vec(csv.collect::<Vec<i32>>())),
                DataStruct::VecDeque => Some(Match::VecDeque(csv.collect::<VecDeque<i32>>())),
            }
        }
        Target::Equation => {
            for eq in equation::EQUATIONS {
                // eq.1 contains a &str of a regular expression like r"some expression"
                if let Some(_) = Regex::new(eq.1).ok()?.captures(line) {
                    let op = Closure(equation(eq.0, line.to_string())?);
                    return Some(Match::Closure(op));
                }
            }
            None
        }
    }
}

pub fn equation(t: EqType, str: String) -> Option<Box<dyn Fn(u64) -> u64>> {
    let operand = get_first_match(str.as_str(), r"[+|-|*|/|%]")?;

    match t {
        EqType::A => {
            let n = get_first_match(str.as_str(), r"\d+")?.parse::<u64>().ok()?;
            match operand {
                "+" => Some(Box::new(move |x| x + n)),
                "-" => Some(Box::new(move |x| x - n)),
                "*" => Some(Box::new(move |x| x * n)),
                "/" => Some(Box::new(move |x| x / n)),
                "%" => Some(Box::new(move |x| x % n)),
                _ => None,
            }
        }
        EqType::B => match operand {
            "+" => Some(Box::new(move |x| x + x)),
            "-" => Some(Box::new(move |x| x - x)),
            "*" => Some(Box::new(move |x| x * x)),
            "/" => Some(Box::new(move |x| x / x)),
            "%" => Some(Box::new(move |x| x % x)),
            _ => None,
        },
    }
}
