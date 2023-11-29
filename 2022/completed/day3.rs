use std::{collections::HashSet};

fn value(c: &char) -> u32 {
    if *c >= 'a' && *c <= 'z' {
        (*c as u8 - b'a' + 1) as u32
    } else if *c >= 'A' && *c <= 'Z' {
        (*c as u8 - b'A' + 27) as u32
    } else {
        0
    }
}

pub fn solution1(content: &String) -> u32{
    let mut res = 0;
    for line in content.split("\n") {
        
        let (h1, h2) = line.split_at(line.len()/2);
        let set1: HashSet<_> = h1.chars().collect();
        let set2: HashSet<_> = h2.chars().collect();
        let c: HashSet<_> = set1.intersection(&set2).collect();
        res += value(c.iter().next().unwrap());
    }

    println!("Solution 1 is : {}",res);
    res
}

pub fn solution2(content: &String) -> u32 {

    let res = content.lines().array_chunks::<3>()
        .map(|[a,b,c]| {
            let common = a.chars()
                          .find(|common_char| {
                            b.contains(*common_char) &&
                            c.contains(*common_char)
                          }).unwrap();
            value(&common) as usize
        }).sum::<usize>();

    println!("Solution 2 is : {}",res);
    res as u32
}