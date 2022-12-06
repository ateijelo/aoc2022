use std::io::{self, BufRead};

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(rangestr: &str) -> Range {
        let s: Vec<String> = String::from(rangestr)
            .split('-')
            .map(|x| x.to_string())
            .collect();
        assert_eq!(s.len(), 2);
        let a = s[0].parse::<i32>().unwrap();
        let b = s[1].parse::<i32>().unwrap();
        Range { start: a, end: b }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<(Range, Range)> {
    let mut pairs = vec![];
    for line in lines {
        let s: Vec<&str> = line.split(',').collect();
        pairs.push((Range::new(s[0]), Range::new(s[1])));
    }
    pairs
}

fn solution(pairs: Vec<(Range, Range)>) -> u32 {
    let mut count = 0;
    for pair in pairs {
        let (a, b) = pair;
        if a.contains(&b) || b.contains(&a) {
            count += 1
        }
    }
    count
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
    fn test_ranges() {
        let a = Range::new("5-15");
        let b = Range::new("10-25");
        let c = Range::new("1-30");
        assert_eq!(a.start, 5);
        assert_eq!(a.end, 15);
        assert!(c.contains(&a));
        assert!(c.contains(&b));
        assert!(!a.contains(&b));
    }

    #[test]
    fn test_example() {
        let lines = [
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ];
        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let pairs = parse_input(&lines);
        assert_eq!(solution(pairs), 2);
    }
}
