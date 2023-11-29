use std::collections::VecDeque;
use util::parser as p;
use util::parser::Target as pt;

struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: usize,
    fail: usize,
    success: usize,
    count: usize,
}

#[allow(unused_variables)]
pub fn sol1(content: &String) -> Option<usize> {
    let params = p::Param {
        block_size: 7,
        match_types: vec![
            pt::Number,
            pt::CSV(p::DataStruct::VecDeque),
            pt::Equation,
            pt::Number,
            pt::Number,
            pt::Number,
        ],
    };

    let mut monkeys: Vec<Monkey> = vec![];

    let parsed = p::parse(content, &params);

    for idx in 0..parsed.len() {
        monkeys.push(Monkey {
            id: parsed[idx][0].as_number()? as usize,
            items: parsed[idx][1]
                .as_vecdeque()?
                .into_iter()
                .map(|x| x as u64)
                .collect(),
            operation: parsed[idx][2].as_closure(),
            test: parsed[idx][3].as_number()? as usize,
            fail: parsed[idx][4].as_number()? as usize,
            success: parsed[idx][5].as_number()? as usize,
            count: 0,
        });
    }

    for r in 0..20 {
        println!("Round {r}");

        for m in 0..monkeys.len() {
            println!("  Monkey {m}");
            for _ in 0..(monkeys[m].items).len() {
                let item = monkeys[m].items.pop_front()?;

                println!("    item : {item}");
                let mut val: u64 = (monkeys[m].operation)(item as u64);

                println!("      after operation : {val}");
                val /= 3;

                println!("      after division : {val}");

                let next_monkey = match val % monkeys[m].test as u64 == 0 {
                    true => monkeys[m].success as usize,
                    false => monkeys[m].fail as usize,
                };

                println!("      Move to monkey : {next_monkey}");
                monkeys[next_monkey].items.push_back(val);
            }
        }
    }

    Some(0)
}

#[allow(unused_variables)]
pub fn sol2(content: &String) -> Option<usize> {
    unimplemented!();
}
