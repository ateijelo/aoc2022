use std::{
    char,
    collections::HashSet,
    io::{self, BufRead},
};

fn letter_value(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<(&str, &str)> {
    let mut pairs = vec![];
    for line in lines {
        pairs.push(line.split_at(line.len() / 2));
    }
    pairs
}

fn solution(pairs: Vec<(&str, &str)>) -> u32 {
    let mut sum = 0;
    for pair in pairs {
        let h1: HashSet<char> = pair.0.chars().collect();
        let h2: HashSet<char> = pair.1.chars().collect();
        let common = h1.intersection(&h2).next().unwrap();
        sum += letter_value(*common);
    }
    sum
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines = lines.map(|line| line.unwrap()).collect();
    println!("{:?}", solution(parse_input(&lines)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(letter_value('a'), 1);
        assert_eq!(letter_value('z'), 26);
        assert_eq!(letter_value('A'), 27);
        assert_eq!(letter_value('Z'), 52);
    }

    #[test]
    fn test_example() {
        let lines = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let pairs = parse_input(&lines);
        assert_eq!(solution(pairs), 157);
    }
}
