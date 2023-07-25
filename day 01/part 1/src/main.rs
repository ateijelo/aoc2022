use std::io;

fn parse_input(lines: Vec<String>) -> Vec<i32> {
    let mut calories = vec![];
    let mut c: i32 = 0;
    for line in lines {
        if line.is_empty() {
            calories.push(c);
            c = 0;
        } else {
            c += line.parse::<i32>().unwrap();
        }
    }
    calories.push(c);
    calories
}

fn solution(calories: Vec<i32>) -> i32 {
    calories.into_iter().max().unwrap_or_default()
}

fn main() {
    let calories = parse_input(io::stdin().lines().map(|line| line.unwrap()).collect());
    println!("{:?}", solution(calories));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec!["1000", "2000", "", "3000", "4000"];
        let calories = parse_input(lines.iter().map(|x| x.to_string()).collect());
        assert_eq!(calories, [3000, 7000]);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(vec![10, 20, 30]), 30);
        assert_eq!(solution(vec![]), 0);
    }
}
