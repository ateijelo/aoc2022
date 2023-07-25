use std::{
    char,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

struct Plan {
    stacks: Vec<String>,
    instructions: Vec<Move>,
}

fn parse_input(lines: &Vec<String>) -> Plan {
    let mut reading_instructions = false;
    let mut rows: Vec<String> = vec![];
    let mut instructions: Vec<Move> = vec![];
    let mut stack_count = 0;
    for line in lines {
        if line.starts_with(" 1") {
            reading_instructions = true;
        }
        if reading_instructions {
            let ws: Vec<&str> = line.split_whitespace().collect();
            if ws.len() != 6 {
                continue;
            }
            let count: usize = ws[1].to_string().parse().unwrap();
            let from: usize = ws[3].to_string().parse().unwrap();
            let to: usize = ws[5].to_string().parse().unwrap();

            instructions.push(Move { count, from, to })
        } else {
            let mut row = "".to_string();
            let mut i = 1;
            let line: Vec<char> = line.chars().collect();
            while i < line.len() {
                row.push(line[i]);
                i += 4;
                stack_count = std::cmp::max(stack_count, row.len());
            }
            rows.push(row);
        }
    }

    // transpose rows into stacks
    let mut stacks: Vec<String> = vec![];
    stacks.resize_with(stack_count, String::default);
    for y in 0..rows.len() {
        let row: Vec<char> = rows[rows.len() - 1 - y].chars().collect();
        for x in 0..stack_count {
            if x >= row.len() {
                continue;
            }
            let c = row[x];
            if c != ' ' {
                stacks[x].push(c);
            }
        }
    }

    Plan {
        stacks,
        instructions,
    }
}

fn solution(plan: &mut Plan) -> String {
    println!("stacks: {:?}", &plan.stacks);
    for instruction in &plan.instructions {
        for _ in 0..instruction.count {
            println!("instruction: {:?}", instruction);
            let c = plan.stacks[instruction.from - 1].pop().unwrap();
            let to = &mut plan.stacks[instruction.to - 1];
            to.push(c);
        }
    }
    let mut result = "".to_string();
    for stack in &mut plan.stacks {
        result.push(stack.chars().last().unwrap())
    }
    result
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines = lines.map(|line| line.unwrap()).collect();
    println!("{:?}", solution(&mut parse_input(&lines)));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ranges() {
    //     let a = Range::new("5-15");
    //     let b = Range::new("10-25");
    //     let c = Range::new("1-30");
    //     assert_eq!(a.start, 5);
    //     assert_eq!(a.end, 15);
    //     assert!(c._contains(&a));
    //     assert!(c._contains(&b));
    //     assert!(!a._contains(&b));
    // }
    //
    // #[test]
    // fn test_overlaps() {
    //     assert!( Range::new("5-7").overlaps(&Range::new("7-9")));
    //     assert!( Range::new("2-8").overlaps(&Range::new("3-7")));
    //     assert!( Range::new("6-6").overlaps(&Range::new("4-6")));
    //     assert!( Range::new("2-6").overlaps(&Range::new("4-8")));
    //     assert!( Range::new("2-2").overlaps(&Range::new("2-2")));
    //
    //     assert!(!Range::new("1-1").overlaps(&Range::new("2-2")));
    //     assert!(!Range::new("1-5").overlaps(&Range::new("6-7")));
    // }

    #[test]
    fn test_parse_stacks() {
        let lines = [
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let plan = parse_input(&lines);
        assert_eq!(plan.stacks, vec!["ZN", "MCD", "P"]);
    }

    #[test]
    fn test_parse_instructions() {
        let lines = [
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let plan = parse_input(&lines);
        assert_eq!(
            plan.instructions,
            vec![
                Move {
                    count: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    count: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    count: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    count: 1,
                    from: 1,
                    to: 2
                },
            ]
        );
    }

    #[test]
    fn test_example() {
        let lines = [
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let mut plan = parse_input(&lines);
        assert_eq!(solution(&mut plan), "CMZ");
        println!("stacks: {:?}", &plan.stacks);
    }
}
