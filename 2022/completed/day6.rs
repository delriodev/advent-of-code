use itertools::Itertools;

pub fn solution1(content: &String) -> usize {

    let mut r = (0,4);
    
    while 
        r.1 <= content.len() && 
        content.get(r.0..r.1).unwrap().chars().unique().count() != 4 {
            r.0 += 1;
            r.1 += 1;
    }
    r.1
}   

pub fn solution2(content: &String) -> usize {
    let mut r = (0,14);
    
    while 
        r.1 <= content.len() && 
        content.get(r.0..r.1).unwrap().chars().unique().count() != 14 {
            r.0 += 1;
            r.1 += 1;
    }
    r.1
}