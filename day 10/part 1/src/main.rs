use std::io::{self, BufRead};

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(lines: &[String]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["noop"] => {
                instructions.push(Instruction::Noop);
            }
            ["addx", value] => {
                instructions.push(Instruction::Addx(value.parse().unwrap()));
            }
            _ => {}
        }
    }
    instructions
}

fn solution(instructions: &Vec<Instruction>) -> i32 {
    let read_points = [20, 60, 100, 140, 180, 220];
    let mut x = 1;
    let mut cycle = 1;
    let mut strength = 0;

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                if read_points.contains(&cycle) {
                    strength += cycle * x;
                }
                cycle += 1;
            }
            Instruction::Addx(value) => {
                for _ in 0..2 {
                    if read_points.contains(&cycle) {
                        strength += cycle * x;
                    }
                    cycle += 1;
                }
                x += value;
            }
        }
    }
    strength
}

fn solve(lines: &[String]) -> i32 {
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

    #[test]
    fn test_example() {
        let reader = BufReader::new(File::open("example.txt").unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines), 13140);
    }
}
