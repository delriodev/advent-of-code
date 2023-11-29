use std::cmp::Ordering;

use itertools::Itertools;

fn is_contained(first: (u8, u8), second: (u8, u8)) -> bool {
    (first.0 <= second.0 && first.1 >= second.1) || (second.0 <= first.0 && second.1 >= first.1)
}

fn overlaps(first: (u8, u8), second: (u8, u8)) -> bool {

    match first.cmp(&second) {
        Ordering::Less => first.1 >= second.0,
        Ordering::Equal => true,
        Ordering::Greater => second.1 >= first.0,
    }
}

pub fn solution(content: &String) -> usize{

    let res = content.lines().map(|line| {

        let (first_elf, second_elf) = line.split(",").map(|elf| {
            
            let (start_section, end_section) = elf.split("-").collect_tuple().unwrap();
        
            (start_section.parse().expect("elf's starting section was not a number"), 
             end_section.parse().expect("elf's ending section was not a number"))

        }).collect_tuple().unwrap();

        match is_contained(first_elf, second_elf) {
            true => 1,
            false => 0
        }

    }).sum::<usize>();
    res
}

pub fn solution2(content: &String) -> usize {

    let res = content.lines().map(|line| {

        let (first_elf, second_elf) = line.split(",").map(|elf| {
            
            let (start_section, end_section) = elf.split("-").collect_tuple().unwrap();
        
            (start_section.parse().expect("elf's starting section was not a number"), 
             end_section.parse().expect("elf's ending section was not a number"))

        }).collect_tuple().unwrap();

        match overlaps(first_elf, second_elf) {
            true => 1,
            false => 0
        }

    }).sum::<usize>();
    res
}