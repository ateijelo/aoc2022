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

struct Group<'a>(&'a str, &'a str, &'a str);

fn parse_input(lines: &Vec<String>) -> Vec<Group> {
    let mut groups = vec![];
    let mut i = 0;
    while i < lines.len() {
        groups.push(Group(
            lines[i].as_str(),
            lines[i + 1].as_str(),
            lines[i + 2].as_str(),
        ));
        i += 3;
    }
    groups
}

fn solution(groups: Vec<Group>) -> u32 {
    let mut sum = 0;
    for group in groups {
        let h1: HashSet<char> = group.0.chars().collect();
        let h2: HashSet<char> = group.1.chars().collect();
        let h3: HashSet<char> = group.2.chars().collect();
        let h1h2: HashSet<char> = h1.intersection(&h2).copied().collect();
        let common = h1h2.intersection(&h3).next().unwrap();
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
        assert_eq!(solution(pairs), 70);
    }
}
