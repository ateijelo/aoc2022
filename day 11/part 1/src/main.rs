use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[derive(Debug)]
enum MonkeyOp {
    Add(u32),
    Mul(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    op: MonkeyOp,
    divisibe_by: u32,
    if_true: usize,
    if_false: usize,
    inspections: u32,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            op: MonkeyOp::Square,
            divisibe_by: 1,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["Monkey", _] => {
                monkeys.push(Monkey::new());
            }
            ["Starting", "items:", ..] => {
                monkeys.last_mut().unwrap().items = parts[2..]
                    .iter()
                    .map(|x| x.replace(',', "").parse().unwrap())
                    .collect();
            }
            ["Operation:", "new", "=", "old", op, arg] => {
                let last_monkey = monkeys.last_mut().unwrap();
                if arg == "old" {
                    continue;
                }
                match op {
                    "+" => {
                        last_monkey.op = MonkeyOp::Add(arg.parse().unwrap());
                    }
                    "*" => {
                        last_monkey.op = MonkeyOp::Mul(arg.parse().unwrap());
                    }
                    _ => {}
                }
            }
            ["Test:", "divisible", "by", number] => {
                monkeys.last_mut().unwrap().divisibe_by = number.parse().unwrap();
            }
            ["If", "true:", "throw", "to", "monkey", number] => {
                monkeys.last_mut().unwrap().if_true = number.parse().unwrap();
            }
            ["If", "false:", "throw", "to", "monkey", number] => {
                monkeys.last_mut().unwrap().if_false = number.parse().unwrap();
            }
            _ => {}
        }
    }
    monkeys
}

fn play_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        loop {
            let mut item;
            let next_monkey;
            {
                let monkey = &mut monkeys[i];
                if monkey.items.is_empty() {
                    break;
                }
                // monkey grabs item, increase worry
                item = monkey.items.pop_front().unwrap();
                monkey.inspections += 1;
                match monkey.op {
                    MonkeyOp::Add(value) => item += value,
                    MonkeyOp::Mul(value) => item *= value,
                    MonkeyOp::Square => item = item * item,
                }
                // monkey gets bored, decrease worry
                item /= 3;
                // perform test
                if item % monkey.divisibe_by == 0 {
                    next_monkey = monkey.if_true;
                } else {
                    next_monkey = monkey.if_false;
                }
            }
            monkeys[next_monkey].items.push_back(item);
        }
    }
}

fn solution(monkeys: &mut Vec<Monkey>) -> u32 {
    for _ in 0..20 {
        play_round(monkeys);
    }
    monkeys.sort_by_key(|monkey| monkey.inspections);
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.inspections)
            .product()
}

fn solve(lines: &[String]) -> u32 {
    solution(&mut parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    // for line in solve(&lines) {
    //     println!("{}", line);
    // }
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_example() {
        let reader = BufReader::new(File::open("example.txt").unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(
            solve(&lines),
            10605
        );
    }
}
