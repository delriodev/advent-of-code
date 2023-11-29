use std::{io::Read, any};

struct Match{
   elf: String,
   me: String
}

impl Match {
   pub fn shapeScore(&self) -> u32 {
      match self.me.as_str() {
         "X" => 1,
         "Y" => 2,
         "Z" => 3,
         any => 0
      }
   }
}

impl Match {
   pub fn matchScore(&self) -> u32 {
      // Elf wins
      if (self.elf == "A" && self.me == "Z") || 
         (self.elf == "B" && self.me == "X") || 
         (self.elf == "C" && self.me == "Y") {
         return 0;
      // I win
      } else if (self.elf == "A" && self.me == "Y") || 
                (self.elf == "B" && self.me == "Z") || 
                (self.elf == "C" && self.me == "X") {
         return 6;
      // Tie          
      } else {
         return 3;
      }
   }
}

impl Match {
   pub fn part2MatchScore(&self) -> u32 {
      match self.me.as_str() {
         "X" => { // I must lose
            match self.elf.as_str() {
               "A" => 3,
               "B" => 1,
               "C" => 2,
               any => 0
            }
         },
         "Y" => { // Must draw
            match self.elf.as_str() {
               "A" => 3 + 1,
               "B" => 3 + 2,
               "C" => 3 + 3,
               any => 0
            }
         },
         "Z" => { // Must win
            match self.elf.as_str() {
               "A" => 6 + 2,
               "B" => 6 + 3,
               "C" => 6 + 1,
               any => 0
            }
         },
         any => 0
      }
   }
}

impl From<&str> for Match {
   fn from(s: &str) -> Self{
      let ss = s.split(" ").collect::<Vec<&str>>();
      Self {
         elf: ss[0].to_string(),
         me: ss[1].to_string()
      }
   }
}

fn main(){
   let mut file = std::fs::File::open("input").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();

   let part1total: u32 = contents
      .split("\n")
      .map(|s| Match::from(s))
      .map(|m| m.shapeScore() + m.matchScore())
      .sum();

   let part2total: u32 = contents
      .split("\n")
      .map(|s| Match::from(s))
      .map(|m| m.part2MatchScore())
      .sum();

   print!("{}", part2total);
}

