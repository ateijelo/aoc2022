use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
enum Phrase {
    Expr(String, String, String),
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
            map.insert(name, Phrase::Expr(lhs, op, rhs));
        }
    }
    map
}

fn evaluate(lv: i64, op: &str, rv: i64) -> i64 {
    println!("evaluating {} {} {}", lv, op, rv);
    match op {
        "+" => lv + rv,
        "-" => lv - rv,
        "*" => lv * rv,
        "/" => lv / rv,
        _ => panic!("invalid operator: {}", op),
    }
}

fn solution(conversation: &Conversation) -> i64 {
    let mut known: HashMap<&String, i64> = HashMap::new();
    let names: Vec<String> = conversation.keys().map(|x| (*x).clone()).collect();
    loop {
        let previously_known = known.len();
        for name in names.iter() {
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
    let r = known.get(&"root".to_string()).unwrap();
    *r
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
        test_file("example.txt", "152");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "21208142603224");
    }
}
