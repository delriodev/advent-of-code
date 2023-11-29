use std::collections::HashMap;
use util::{cardinal_direction as dir, parser};

fn from_cmd(s: &str) -> Option<dir::Direction> {
    match s {
        "U" => Some(dir::Direction::N),
        "R" => Some(dir::Direction::E),
        "D" => Some(dir::Direction::S),
        "L" => Some(dir::Direction::W),
        _ => None,
    }
}

fn next_knot_movement(head: &dir::Coord, tail: &dir::Coord) -> Option<dir::Direction> {
    let dist = head.abs_distance(tail);
    let dir = tail.relative_direction(head);

    if dist.0 <= 1 && dist.1 <= 1 {
        return None;
    } else {
        return dir;
    }
}

#[allow(unused_variables)]
pub fn sol1(content: &String) -> usize {
    let mut visited: HashMap<dir::Coord, _> = HashMap::new();

    let mut head = dir::Coord { x: 0, y: 0 };
    let mut tail = dir::Coord { x: 0, y: 0 };

    for line in content.lines() {
        let (command, reps) = parser::str_num(line).unwrap();
        for _ in 0..reps {
            head.displace(from_cmd(command).unwrap(), 1);
            match next_knot_movement(&head, &tail) {
                Some(c) => tail.displace(c, 1),
                None => {}
            }
            visited.insert(tail, 0);
        }
    }
    visited.len()
}

#[allow(unused_variables)]
pub fn sol2(content: &String) -> usize {
    let mut visited: HashMap<dir::Coord, _> = HashMap::new();
    let mut knots: [dir::Coord; 10] = [dir::Coord { x: 0, y: 0 }; 10];

    for line in content.lines() {
        let (command, repetitions) = parser::str_num(line).unwrap();
        for _ in 0..repetitions {
            knots[0].displace(from_cmd(command).unwrap(), 1);
            for i in 1..10 {
                match next_knot_movement(&knots[i - 1], &knots[i]) {
                    Some(c) => knots[i].displace(c, 1),
                    None => break,
                }
            }
            visited.insert(knots[9].clone(), 0);
        }
    }
    visited.len()
}
