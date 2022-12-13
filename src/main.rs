use std::io::{self, BufRead};

use pathfinding::prelude::dijkstra;

struct Input {
    map: Vec<Vec<u32>>,
    start: Pos,
    end: Pos,
}

fn parse_input(lines: &[String]) -> Input {
    let mut input = Input {
        map: vec![],
        start: Pos { x: 0, y: 0 },
        end: Pos { x: 0, y: 0 },
    };
    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<u32> = vec![];
        for (x, c) in line.chars().enumerate() {
            let v = match c {
                'S' => {
                    input.start = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                    0
                }
                'E' => {
                    input.end = Pos {
                        x: x as i32,
                        y: y as i32,
                    };
                    'z' as u32 - 'a' as u32
                }
                'a'..='z' => c as u32 - 'a' as u32,
                _ => 0,
            };
            row.push(v);
        }
        input.map.push(row);
    }
    input
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn successors(pos: &Pos, map: &[Vec<u32>]) -> Vec<(Pos, usize)> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut v: Vec<(Pos, usize)> = vec![];
    for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        if pos.x + dx >= width {
            continue;
        }
        if pos.y + dy >= height {
            continue;
        }
        if pos.x + dx < 0 {
            continue;
        }
        if pos.y + dy < 0 {
            continue;
        }
        let new_pos = Pos {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        let mut cost = 4000;
        let current_height = map[pos.y as usize][pos.x as usize];
        let neighbor_height = map[new_pos.y as usize][new_pos.x as usize];
        if current_height + 1 >= neighbor_height {
            cost = 1;
        }
        v.push((new_pos, cost));
    }
    v
}

fn solution(input: Input) -> usize {
    let mut min = 4000;
    for (y, row) in input.map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != 0 { continue; }
            let Some(result) = dijkstra(
                &Pos { x: x as i32, y: y as i32 },
                |p| successors(p, &input.map),
                |p| *p == input.end,
            ) else { continue };
            println!("From ({}, {}): {}", x, y, result.1);
            min = std::cmp::min(min, result.1);
        }
    }

    min
}

fn solve(lines: &[String]) -> usize {
    solution(parse_input(lines))
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
        assert_eq!(solve(&lines), 29);
    }
}
