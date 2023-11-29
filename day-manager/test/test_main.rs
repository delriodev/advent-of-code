const TEST_1_RESULT: usize = 0;
const TEST_2_RESULT: usize = 0;

use std::{io::Read};
mod dayx;
use dayx as sol;

fn main(){
    let mut input_file = std::fs::File::open("input").unwrap();
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content).unwrap();

    println!("Result of part 1 is : {:?}", sol::sol1(&input_content));
    println!("Result of part 2 is : {:?}", sol::sol2(&input_content));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prep() -> String {
        let mut test = std::fs::File::open("test").unwrap();
        let mut content = String::new();
        test.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn sol1() {
        assert_eq!(sol::sol1(&prep()), TEST_1_RESULT);
    }
    #[test]
    fn sol2() {
        assert_eq!(sol::sol2(&prep()), TEST_2_RESULT);
    }
}