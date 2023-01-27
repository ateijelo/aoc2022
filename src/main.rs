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

fn next_coords(x: usize, y: usize, dir: char) -> (usize, usize, char) {
    let directions = HashMap::from([
        ('v', Vector::from((0, 1))),
        ('^', Vector::from((0, -1))),
        ('>', Vector::from((1, 0))),
        ('<', Vector::from((-1, 0))),
    ]);

    match (x, y, dir) {
        // red edge
        (50, 50..=99, '<') => (y - 50, 100, 'v'),
        (0..=49, 100, '^') => (50, x + 50, '>'),

        // orange edge
        (50, 0..=49, '<') => (0, 149 - y, '>'),
        (0, 100..=149, '<') => (50, 149 - y, '>'),

        // yellow edge
        (50..=99, 0, '^') => (0, x + 100, '>'),
        (0, 150..=199, '<') => (y - 100, 0, 'v'),

        // green edge
        (100..=149, 0, '^') => (x - 100, 199, '^'),
        (0..=49, 199, 'v') => (x + 100, 0, 'v'),

        // cyan edge
        (49, 150..=199, '>') => (y - 100, 149, '^'),
        (50..=99, 149, 'v') => (49, x + 100, '<'),

        // blue edge
        (99, 100..=149, '>') => (149, 149 - y, '<'),
        (149, 0..=49, '>') => (99, 149 - y, '<'),

        // purple edge
        (99, 50..=99, '>') => (y + 50, 49, '^'),
        (100..=149, 49, 'v') => (99, x - 50, '<'),

        (_, _, _) => {
            let nx = (x as i32 + directions.get(&dir).unwrap().x) as usize;
            let ny = (y as i32 + directions.get(&dir).unwrap().y) as usize;
            (nx, ny, dir)
        }
    }
}

fn solution(input: &mut Input) -> usize {
    let mut x = input.map[0].iter().position(|c| *c == Cell::Dot).unwrap();
    let mut y = 0usize;
    let mut dir = '>';
    input.map[y][x] = Cell::Dir(dir);
    for step in input.steps.iter() {
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
                while amount > 0 {
                    let (nx, ny, ndir) = next_coords(x, y, dir);
                    match input.map[ny][nx] {
                        Cell::Blank => {
                            panic!("should never happen!");
                        }
                        Cell::Dot | Cell::Dir(_) => {
                            amount -= 1;
                            x = nx;
                            y = ny;
                            dir = ndir;
                            input.map[y][x] = Cell::Dir(dir);
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
        _ => panic!("bad dir"),
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

        let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    #[ignore]
    fn test_example() {
        test_file("example.txt", "6032");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "124302");
    }

    #[test]
    fn test_next_coord() {
        // internal movement
        assert_eq!(next_coords(50, 0, '>'), (51, 0, '>'));
        assert_eq!(next_coords(50, 0, 'v'), (50, 1, 'v'));
        assert_eq!(next_coords(49, 149, '>'), (50, 149, '>'));
        assert_eq!(next_coords(49, 149, 'v'), (49, 150, 'v'));
        assert_eq!(next_coords(49, 149, '<'), (48, 149, '<'));
        assert_eq!(next_coords(49, 149, '^'), (49, 148, '^'));

        // red edge
        assert_eq!(next_coords(50, 0, '<'), (0, 149, '>'));
        assert_eq!(next_coords(50, 1, '<'), (0, 148, '>'));
        assert_eq!(next_coords(49, 100, '^'), (50, 99, '>'));
        assert_eq!(next_coords(48, 100, '^'), (50, 98, '>'));

        // orange edge
        assert_eq!(next_coords(50, 0, '<'), (0, 149, '>'));
        assert_eq!(next_coords(50, 1, '<'), (0, 148, '>'));
        assert_eq!(next_coords(0, 100, '<'), (50, 49, '>'));
        assert_eq!(next_coords(0, 101, '<'), (50, 48, '>'));

        // yellow edge
        assert_eq!(next_coords(50, 0, '^'), (0, 150, '>'));
        assert_eq!(next_coords(51, 0, '^'), (0, 151, '>'));
        assert_eq!(next_coords(0, 150, '<'), (50, 0, 'v'));
        assert_eq!(next_coords(0, 151, '<'), (51, 0, 'v'));

        // green edge
        assert_eq!(next_coords(100, 0, '^'), (0, 199, '^'));
        assert_eq!(next_coords(101, 0, '^'), (1, 199, '^'));
        assert_eq!(next_coords(0, 199, 'v'), (100, 0, 'v'));
        assert_eq!(next_coords(1, 199, 'v'), (101, 0, 'v'));

        // cyan edge
        assert_eq!(next_coords(49, 150, '>'), (50, 149, '^'));
        assert_eq!(next_coords(49, 151, '>'), (51, 149, '^'));
        assert_eq!(next_coords(99, 149, 'v'), (49, 199, '<'));
        assert_eq!(next_coords(98, 149, 'v'), (49, 198, '<'));

        // blue edge
        assert_eq!(next_coords(99, 100, '>'), (149, 49, '<'));
        assert_eq!(next_coords(99, 101, '>'), (149, 48, '<'));
        assert_eq!(next_coords(149, 0, '>'), (99, 149, '<'));
        assert_eq!(next_coords(149, 1, '>'), (99, 148, '<'));

        // purple edge
        assert_eq!(next_coords(99, 50, '>'), (100, 49, '^'));
        assert_eq!(next_coords(99, 51, '>'), (101, 49, '^'));
        assert_eq!(next_coords(100, 49, 'v'), (99, 50, '<'));
        assert_eq!(next_coords(101, 49, 'v'), (99, 51, '<'));
    }
}
