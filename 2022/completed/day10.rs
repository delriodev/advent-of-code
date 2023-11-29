// use itertools:Itertools;
use util::parser;

struct CPU {
    cycle: usize,
    registry: i32,
}

impl CPU {
    fn add_cycle(&mut self) -> Option<i32> {
        if [20, 60, 100, 140, 180, 220].contains(&self.cycle) {
            let ret = self.cycle as i32 * &self.registry;
            self.cycle += 1;
            return Some(ret);
        }
        self.cycle += 1;
        None
    }

    fn increment_cycle(&mut self) {
        self.cycle += 1;
    }

    fn add_to_registry(&mut self, value: i32) {
        self.registry += value;
    }
}

#[allow(unused_variables)]
pub fn sol1(content: &String) -> i32 {
    let mut cpu = CPU {
        cycle: 1,
        registry: 1,
    };
    let mut signal = 0;
    for line in content.lines() {
        let (command, value) = parser::str_num(line).unwrap();
        match command {
            "noop" => {
                signal += cpu.add_cycle().unwrap_or_else(|| 0);
            }
            "addx" => {
                signal += cpu.add_cycle().unwrap_or_else(|| 0);
                signal += cpu.add_cycle().unwrap_or_else(|| 0);
                cpu.add_to_registry(value.unwrap());
            }
            _ => {}
        }
    }

    signal
}

struct CRT {
    pixel: usize,
    screen: Vec<Vec<char>>,
}

impl CRT {
    fn draw_pixel(&mut self, sprite: i32) {
        let col = self.pixel % 40;

        if col as i32 >= sprite - 1 && col as i32 <= sprite + 1 {
            self.screen[self.pixel / 40][col] = '#';
        }
    }
    fn move_pixel(&mut self) {
        if self.pixel < 239 {
            self.pixel += 1;
        } else {
            self.pixel = 0;
        }
    }
}

#[allow(unused_variables)]
pub fn sol2(content: &String) -> usize {
    let mut cpu = CPU {
        cycle: 1,
        registry: 1,
    };
    let mut crt = CRT {
        pixel: 0,
        screen: vec![vec!['.'; 40]; 6],
    };

    for line in content.lines() {
        let (command, value) = parser::str_num(line).unwrap();
        match command {
            "noop" => {
                crt.draw_pixel(cpu.registry);
                crt.move_pixel();
                cpu.increment_cycle();
            }
            "addx" => {
                crt.draw_pixel(cpu.registry);
                crt.move_pixel();
                cpu.increment_cycle();
                crt.draw_pixel(cpu.registry);
                crt.move_pixel();
                cpu.increment_cycle();
                cpu.add_to_registry(value.unwrap());
            }
            _ => {}
        }
    }

    for row in crt.screen {
        for c in row {
            print!(" {}", c);
        }
        println!();
    }
    0
}
