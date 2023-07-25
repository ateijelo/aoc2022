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

fn render(x: i32, cycle: i32, scanlines: &mut Vec<String>) {
    let pixel = (cycle - 1) % 40;
    if pixel == 0 {
        scanlines.push("".to_string());
    }
    if i32::abs(x - pixel) <= 1 {
        scanlines.last_mut().unwrap().push('#');
    } else {
        scanlines.last_mut().unwrap().push('.');
    }
}

fn solution(instructions: &Vec<Instruction>) -> Vec<String>{
    let mut x = 1;
    let mut cycle = 1;
    let mut scanlines: Vec<String> = vec![];

    for instr in instructions {
        match instr {
            Instruction::Noop => {
                render(x, cycle, &mut scanlines);
                cycle += 1;
            }
            Instruction::Addx(value) => {
                for _ in 0..2 {
                    render(x, cycle, &mut scanlines);
                    cycle += 1;
                }
                x += value;
            }
        }
    }
    scanlines
}

fn solve(lines: &[String]) -> Vec<String> {
    solution(&parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    for line in solve(&lines) {
        println!("{}", line);
    }
    // println!("{}", solve(&lines));
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
        assert_eq!(solve(&lines), vec![
            "##..##..##..##..##..##..##..##..##..##..".to_string(),
            "###...###...###...###...###...###...###.".to_string(),
            "####....####....####....####....####....".to_string(),
            "#####.....#####.....#####.....#####.....".to_string(),
            "######......######......######......####".to_string(),
            "#######.......#######.......#######.....".to_string(),
        ]);
    }
}
