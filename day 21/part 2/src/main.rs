use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("invalid operator: {}", s),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
enum Phrase {
    Expr(String, Op, String),
    Number(i64),
}

type Conversation = HashMap<String, Phrase>;

fn parse_input(lines: &[String]) -> HashMap<String, Phrase> {
    let mut map = HashMap::new();
    let r1 = Regex::new(r"(\w+): (\d+)$").unwrap();
    let r2 = Regex::new(r"(\w+): (\w+) ([+*/-]) (\w+)").unwrap();

    for line in lines {
        if let Some(caps) = r1.captures(line) {
            let name: String = caps.get(1).unwrap().as_str().to_string();
            let value: i64 = caps.get(2).unwrap().as_str().parse().unwrap();
            map.insert(name, Phrase::Number(value));
        }
        if let Some(caps) = r2.captures(line) {
            let name: String = caps.get(1).unwrap().as_str().to_string();
            let lhs: String = caps.get(2).unwrap().as_str().to_string();
            let op: String = caps.get(3).unwrap().as_str().to_string();
            let rhs: String = caps.get(4).unwrap().as_str().to_string();
            map.insert(name, Phrase::Expr(lhs, Op::from(&op), rhs));
        }
    }
    map
}

fn evaluate(lv: i64, op: &Op, rv: i64) -> i64 {
    match op {
        Op::Add => lv + rv,
        Op::Sub => lv - rv,
        Op::Mul => lv * rv,
        Op::Div => lv / rv,
    }
}

fn walk(
    conversation: &Conversation,
    known: &HashMap<&String, i64>,
    name: &str,
    expr: i64,
) -> Option<i64> {
    let p = conversation.get(name).unwrap();
    match p {
        Phrase::Expr(lhs, op, rhs) => {
            if let Some(lv) = known.get(lhs) {
                let rv = match op {
                    Op::Add => expr - lv, // expr = lv + x, x = expr - lv
                    Op::Sub => lv - expr, // expr = lv - x, x = lv - expr
                    Op::Mul => expr / lv, // expr = lv * x, x = expr / lv
                    Op::Div => lv / expr, // expr = lv / x, x = lv / expr
                };
                if rhs == "humn" {
                    return Some(rv);
                }
                return walk(conversation, known, rhs, rv);
            }
            if let Some(rv) = known.get(rhs) {
                let lv = match op {
                    Op::Add => expr - rv, // expr = x + rv, x = expr - rv
                    Op::Sub => expr + rv, // expr = x - rv, x = expr + rv
                    Op::Mul => expr / rv, // expr = x * rv, x = expr / rv
                    Op::Div => expr * rv, // expr = x / rv, x = expr * rv
                };
                if lhs == "humn" {
                    return Some(lv);
                }
                return walk(conversation, known, lhs, lv);
            }
            None
        }
        Phrase::Number(_) => None,
    }
}

fn solution(conversation: &Conversation) -> i64 {
    let mut known: HashMap<&String, i64> = HashMap::new();

    let names: Vec<String> = conversation.keys().map(|x| (*x).clone()).collect();
    loop {
        let previously_known = known.len();
        for name in names.iter() {
            if name == "humn" {
                continue;
            }
            if known.get(name).is_none() {
                let value = conversation.get(name).unwrap();

                match value {
                    Phrase::Number(v) => {
                        known.insert(name, *v);
                    }
                    Phrase::Expr(lhs, op, rhs) => {
                        if let Some(lv) = known.get(lhs) {
                            if let Some(rv) = known.get(rhs) {
                                let r = evaluate(*lv, op, *rv);
                                known.insert(name, r);
                            }
                        }
                    }
                }
            }
        }
        if known.contains_key(&"root".to_string()) {
            break;
        }
        if known.len() == previously_known {
            break;
        }
    }
    let p = conversation.get("root").unwrap();
    let Phrase::Expr(lhs, _, rhs) = p else {
        panic!("root is a number!");
    };
    if let Some(lv) = known.get(lhs) {
        // we know lhs, let's solve for rhs = lv
        return walk(conversation, &known, rhs, *lv).unwrap();
    }
    if let Some(rv) = known.get(rhs) {
        // we know rhs, let's solve for lhs = rv
        return walk(conversation, &known, lhs, *rv).unwrap();
    }
    0
}

fn solve(lines: &[String]) -> i64 {
    solution(&parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn test_file(filename: &str, solution: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "301");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "3882224466191");
    }
}
