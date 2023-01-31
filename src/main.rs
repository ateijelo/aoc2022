use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead},
    ops::Add,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Input {
    start: Point,
    end: Point,
    width: usize,  // blocks between the walls
    height: usize, // blocks between the walls

    left_blizzards: HashSet<Point>,
    right_blizzards: HashSet<Point>,
    up_blizzards: HashSet<Point>,
    down_blizzards: HashSet<Point>,
}

fn parse_input(lines: &[String]) -> Input {
    let lines: Vec<&String> = lines.iter().collect();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    let mut left_blizzards = HashSet::new();
    let mut right_blizzards = HashSet::new();
    let mut up_blizzards = HashSet::new();
    let mut down_blizzards = HashSet::new();
    let height = lines.len() - 2;
    let mut width = 0;
    for (y, line) in lines.iter().enumerate() {
        let y = y as i32;
        width = line.len() - 2;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            match c {
                // '#' => {}
                '.' => {
                    if y == 0 {
                        start = Point { x, y };
                    }
                    if y == lines.len() as i32 - 1 {
                        end = Point { x, y };
                    }
                }
                '<' => {
                    left_blizzards.insert(Point { x, y });
                }
                '>' => {
                    right_blizzards.insert(Point { x, y });
                }
                'v' => {
                    down_blizzards.insert(Point { x, y });
                }
                '^' => {
                    up_blizzards.insert(Point { x, y });
                }
                _ => {}
            }
        }
    }
    Input {
        start,
        end,
        width,
        height,
        left_blizzards,
        right_blizzards,
        up_blizzards,
        down_blizzards,
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    step: u32,
    pos: Point,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Visit {
    point: Point,
    blizzard_state: u32,
}

impl Visit {
    fn new(point: Point, blizzard_state: u32) -> Self {
        Self {
            point,
            blizzard_state,
        }
    }
}

struct Solver<'a> {
    visited: HashSet<Visit>,
    input: &'a Input,
    start: Point,
    end: Point,
    cycle: u32,
}

impl<'a> Solver<'a> {
    fn new(input: &'a Input, start: Point, end: Point) -> Self {
        Self {
            visited: HashSet::new(),
            input,
            start,
            end,
            cycle: num::integer::lcm(input.width as u32, input.height as u32),
        }
    }

    fn any_blizzards_at(&self, point: &Point, step: u32) -> bool {
        if *point == self.start {
            return false;
        }
        if *point == self.end {
            return false;
        }
        if self.out_of_bounds(point) {
            return false;
        }
        let lx = (point.x - 1 + step as i32).rem_euclid(self.input.width as i32) + 1;
        let rx = (point.x - 1 - step as i32).rem_euclid(self.input.width as i32) + 1;
        let uy = (point.y - 1 + step as i32).rem_euclid(self.input.height as i32) + 1;
        let dy = (point.y - 1 - step as i32).rem_euclid(self.input.height as i32) + 1;

        let rb = &self.input.right_blizzards;
        let lb = &self.input.left_blizzards;
        let db = &self.input.down_blizzards;
        let ub = &self.input.up_blizzards;

        if rb.contains(&Point::new(rx, point.y)) {
            return true;
        }
        if lb.contains(&Point::new(lx, point.y)) {
            return true;
        }
        if ub.contains(&Point::new(point.x, uy)) {
            return true;
        }
        if db.contains(&Point::new(point.x, dy)) {
            return true;
        }
        false
    }

    fn mark_visit(&mut self, point: Point, step: u32) {
        self.visited
            .insert(Visit::new(point, step.rem_euclid(self.cycle)));
    }

    fn has_visited(&self, point: Point, step: u32) -> bool {
        self.visited
            .contains(&Visit::new(point, step.rem_euclid(self.cycle)))
    }

    fn out_of_bounds(&self, point: &Point) -> bool {
        if *point == self.start {
            return false;
        }
        if *point == self.end {
            return false;
        }
        if point.x < 1 || point.x > self.input.width as i32 {
            return true;
        }
        if point.y < 1 || point.y > self.input.height as i32 {
            return true;
        }
        false
    }

    fn solve(&mut self, step: u32) -> u32 {
        let moves = [
            ("d", Point::new(0, 1)),
            ("r", Point::new(1, 0)),
            ("w", Point::new(0, 0)),
            ("l", Point::new(-1, 0)),
            ("u", Point::new(0, -1)),
        ];
        let mut q = VecDeque::new();
        q.push_back(State { pos: self.start, step });
        while !q.is_empty() {
            let s = q.pop_front().unwrap();
            if s.pos == self.end {
                return s.step;
            }

            for (_, dir) in moves {
                let np = s.pos + dir;
                let ns = s.step + 1;
                if self.has_visited(np, ns) {
                    continue;
                }
                self.mark_visit(np, ns);
                let ns = s.step + 1;
                if self.out_of_bounds(&np) {
                    continue;
                }
                if self.any_blizzards_at(&np, ns) {
                    continue;
                }
                q.push_back(State { pos: np, step: ns });
            }
        }
        step
    }
}

fn solution(input: &Input) -> u32 {
    let mut solver_a = Solver::new(input, input.start, input.end);
    let mut solver_b = Solver::new(input, input.end, input.start);
    let mut solver_c = Solver::new(input, input.start, input.end);
    let a = solver_a.solve(0);
    let b = solver_b.solve(a);
    solver_c.solve(b)
}

fn solve(lines: &[String]) -> u32 {
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

    fn test_file(filename: &str, solution: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());

        let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "54");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "739");
    }

    #[test]
    fn test_blizzard() {
        let lines = ["#.######", "#<.....#", "######.#"].map(|x| x.to_string());

        let input = parse_input(&lines);
        let p = Point::new(1, 0);
        let solver = Solver::new(&input, p, p);

        // positive cases
        assert!(solver.any_blizzards_at(&Point::new(1, 1), 0));
        assert!(solver.any_blizzards_at(&Point::new(6, 1), 1));

        // negative cases, the ! before the `solve` is hard to see
        assert!(!solver.any_blizzards_at(&Point::new(1, 1), 1));
    }
}
