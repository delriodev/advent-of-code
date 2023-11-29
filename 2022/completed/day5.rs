use std::{collections::VecDeque, iter, vec};
use itertools::Itertools;
use regex::Regex;

pub struct Step {
    n: usize,
    from: usize,
    to: usize
}

pub fn read_drawing(content: &String) -> Vec<&str> {
    content.lines().filter(|&line| line.contains('[')).collect::<Vec<&str>>()
}

pub fn get_last_stack_number(content: &String) -> usize {
    content.lines().filter(
        |line| line.chars().all(
            |c| c.is_numeric() || c.is_whitespace())).next().unwrap().chars().map(
                |d| d.to_digit(10).unwrap_or_else(|| 0)).max().unwrap_or_else(|| 0) as usize
}

pub fn read_procedures(content: &String) -> Vec<Step> {

    let re = Regex::new(r"\d+").unwrap();

    content.lines().filter(|&line| line.contains("move")).map(
        |ins| {
            let i: (usize,usize,usize) = re.captures_iter(ins).map(
                |capture| capture[0].parse().unwrap()).take(3).collect_tuple().unwrap();
            Step {n: i.0, from: i.1 - 1, to: i.2 - 1}
        }).collect()
}

pub fn initialize_puzzle(content: &String) -> Vec<VecDeque<char>> {
    
    let drawing = read_drawing(content);
    let number_of_stacks = get_last_stack_number(content);
    let mut deques: Vec<VecDeque<char>> = iter::repeat(VecDeque::new()).take(number_of_stacks as usize).collect();

    for line in drawing {
        for (i,c) in line.chars().enumerate() {
            if c >= 'A' && c <= 'Z' {
                deques[i/4].push_back(c);
            }
        }
    }
    deques
}

pub fn procede(mut cargo: Vec<VecDeque<char>>, procedure: Vec<Step>) -> Vec<VecDeque<char>> {
    
    for step in procedure {
        for _ in 0..step.n {
            let elf_crate = cargo[step.from].pop_front().unwrap();
            cargo[step.to].push_front(elf_crate);
        }
    }

    cargo
}

pub fn procede2(mut cargo: Vec<VecDeque<char>>, procedure: Vec<Step>) -> Vec<VecDeque<char>> {

    for step in procedure {

        let mut displaced: Vec<char> = vec![];

        for _ in 0..step.n {
            displaced.push(cargo[step.from].pop_front().unwrap());
        }
        displaced.reverse();
        for c in displaced {
            cargo[step.to].push_front(c);
        }
    }
    cargo
}

pub fn solution1(content: &String) -> String{

    let initial_cargo = initialize_puzzle(content);
    let procedures = read_procedures(content);
    let transformed_cargo = procede(initial_cargo.clone(), procedures);

    transformed_cargo.into_iter().map(|mut deq| deq.pop_front().unwrap_or_else(|| '_')).collect::<String>()
}

pub fn solution2(content: &String) -> String {

    let initial_cargo = initialize_puzzle(content);
    let procedures = read_procedures(content);
    let cargo = procede2(initial_cargo.clone(), procedures);

    cargo.into_iter().map(|mut deq| deq.pop_front().unwrap_or_else(|| '_')).collect::<String>()
}