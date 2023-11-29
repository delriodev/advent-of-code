use std::io::Read;
mod day121;
use day121 as sol;

#[allow(dead_code)]
const SOL1: usize = 00605;
#[allow(dead_code)]
const SOL2: usize = 0;

fn main() {
    println!("Advent of code !");
    let mut file = std::fs::File::open("input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let res1 = sol::sol1(&contents);
    println!("Result of part 1 is : {:?}", res1);

    let res2 = sol::sol2(&contents);
    println!("Result of part 2 is : {:?}", res2);
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
    fn solution1() {
        assert_eq!(sol::sol1(&prep()).unwrap(), SOL1);
    }
    #[test]
    fn solution2() {
        assert_eq!(sol::sol2(&prep()).unwrap(), SOL2);
    }
}
