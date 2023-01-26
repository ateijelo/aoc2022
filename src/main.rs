use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Blank,
    Dot,
    Wall,
    Dir(char),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Blank => ' ',
                Cell::Dot => '.',
                Cell::Wall => '#',
                Cell::Dir(c) => *c,
            }
        )
    }
}

#[derive(Debug)]
enum Move {
    Right,
    Left,
    Walk(usize),
}

struct Input {
    map: Vec<Vec<Cell>>,
    steps: Vec<Move>,
}

fn parse_input(lines: &[String]) -> Input {
    let mut steps = Vec::new();
    let mut map: Vec<Vec<Cell>> = Vec::new();
    let mut max_length = 0;
    for line in lines {
        if line.contains('L') {
            let re = Regex::new(r"\d+|L|R").unwrap();
            for m in re.find_iter(line) {
                steps.push(match m.as_str() {
                    "L" => Move::Left,
                    "R" => Move::Right,
                    v => Move::Walk(v.parse().unwrap()),
                });
            }
        }
        if line.contains('.') || line.contains('#') {
            max_length = std::cmp::max(max_length, line.len());
            map.push(
                line.chars()
                    .map(|c| match c {
                        ' ' => Cell::Blank,
                        '.' => Cell::Dot,
                        '#' => Cell::Wall,
                        _ => panic!("Invalid input"),
                    })
                    .collect(),
            )
        }
    }
    for line in map.iter_mut() {
        line.extend([Cell::Blank].repeat(max_length - line.len()))
    }
    // println!("map: {:?}", map);
    // println!("steps: {:?}", steps);
    Input { map, steps }
}

struct Vector {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Vector {
    fn from(value: (i32, i32)) -> Self {
        Vector {
            x: value.0,
            y: value.1,
        }
    }
}

fn solution(input: &mut Input) -> usize {
    let mut x = input.map[0].iter().position(|c| *c == Cell::Dot).unwrap();
    let mut y = 0usize;
    let directions = HashMap::from([
        ('v', Vector::from((0, 1))),
        ('^', Vector::from((0, -1))),
        ('>', Vector::from((1, 0))),
        ('<', Vector::from((-1, 0))),
    ]);
    let mut dir = '>';
    input.map[y][x] = Cell::Dir(dir);
    for step in input.steps.iter() {
        // println!("Doing step: {:?}", step);
        // dbg!(step);
        match *step {
            Move::Right => {
                dir = match dir {
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    '^' => '>',
                    _ => panic!("what?"),
                };
            }
            Move::Left => {
                dir = match dir {
                    'v' => '>',
                    '<' => 'v',
                    '^' => '<',
                    '>' => '^',
                    _ => panic!("what?"),
                }
            }
            Move::Walk(mut amount) => {
                // dbg!(amount);
                let mut tx = x;
                let mut ty = y;
                while amount > 0 {
                    // dbg!(amount);
                    let nx = (tx as i32 + directions.get(&dir).unwrap().x)
                        .rem_euclid(input.map[y].len() as i32)
                        as usize;
                    let ny = (ty as i32 + directions.get(&dir).unwrap().y)
                        .rem_euclid(input.map.len() as i32) as usize;
                    // if input.map[ny][nx] == Cell::Blank {
                    //     continue;
                    // }
                    // dbg!(x, y, nx, ny);
                    // if x == nx && y == ny {
                    //     println!("stuck at {} {}", nx, ny);
                    //     break;
                    // }
                    match input.map[ny][nx] {
                        Cell::Blank => {
                            // dbg!(tx, ty);
                            tx = nx;
                            ty = ny;
                        }
                        Cell::Dot | Cell::Dir(_) => {
                            amount -= 1;
                            // print!("at {} {}, ", x, y);
                            x = nx;
                            y = ny;
                            tx = nx;
                            ty = ny;
                            // println!("moving to {} {}", x, y);
                            input.map[ny][nx] = Cell::Dir(dir);
                        }
                        Cell::Wall => {
                            break;
                        }
                    }
                }
            }
        }

        input.map[y][x] = Cell::Dir(dir);
    }
    for line in input.map.iter() {
        for c in line.iter() {
            print!(
                "{}",
                match c {
                    Cell::Blank => ' ',
                    Cell::Dot => '.',
                    Cell::Wall => '#',
                    Cell::Dir(c) => *c,
                }
            );
        }
        println!()
    }
    println!("final: {} {}, facing {}", x, y, dir);
    let d = match dir {
        '>' => 0,
        'v' => 1,
        '<' => 2,
        '^' => 3,
        _ => panic!("bad dir")
    };
    1000 * (y + 1) + 4 * (x + 1) + d
}

fn solve(lines: &[String]) -> usize {
    solution(&mut parse_input(lines))
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
            .map(|x| x.unwrap())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "6032");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "88268");
    }
}
