use std::{io::{self, BufRead}, collections::HashSet};

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn walk(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => { self.y += 1 }
            Direction::Right => { self.x += 1 },
            Direction::Down => { self.y -= 1 },
            Direction::Left => { self.x -= 1 },
        }
    }

    fn follow(&mut self, other: &Point) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        if i32::abs(dx) > 1 || i32::abs(dy) > 1 {
            self.x += i32::signum(dx);
            self.y += i32::signum(dy);
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<(Direction, u32)> {
    let mut steps: Vec<(Direction, u32)> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let d = match parts[0] {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => { panic!() }
        };
        let c: u32 = parts[1].parse().unwrap();
        steps.push((d, c));
    }
    steps
}


fn solution(steps: &Vec<(Direction, u32)>) -> usize {
    let rope_length = 2;
    let mut knots: Vec<Point> = vec![Point { x:0, y:0 }; rope_length];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (direction, count) in steps {
        for _ in 0..*count {
            knots[0].walk(direction);
            for i in 1..rope_length {
                let t = knots[i-1].clone();
                knots[i].follow(&t);
            }

        visited.insert((knots.last().unwrap().x, knots.last().unwrap().y));
        }
    }
    visited.len()
}

fn solve(lines: &[String]) -> usize {
    solution(&parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow() {
        let mut head;
        let mut tail;

        head = Point { x: 0, y: 0 };
        tail = Point { x: 0, y: 0 };
        head.walk(&Direction::Up);
        tail.follow(&head);
        assert_eq!(tail, Point { x: 0, y: 0 });

        head = Point { x: 0, y: 0 };
        tail = Point { x: 0, y: 0 };
        head.walk(&Direction::Up);
        head.walk(&Direction::Up);
        tail.follow(&head);
        assert_eq!(tail, Point { x: 0, y: 1 });

        head = Point { x: 0, y: 0 };
        tail = Point { x: 0, y: 0 };
        head.walk(&Direction::Up);
        head.walk(&Direction::Right);
        tail.follow(&head);
        assert_eq!(tail, Point { x: 0, y: 0 });

        head = Point { x: 0, y: 0 };
        tail = Point { x: 0, y: 0 };
        head.walk(&Direction::Up);
        head.walk(&Direction::Right);
        head.walk(&Direction::Up);
        tail.follow(&head);
        assert_eq!(tail, Point { x: 1, y: 1 });
    }

    #[test]
    fn test_example() {
        let lines = "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        ";

        let lines: Vec<String> = lines
            .lines()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines), 1);
    }

    #[test]
    fn test_example_two() {
        let lines = "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        ";

        let lines: Vec<String> = lines
            .lines()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines), 36);
    }
}
