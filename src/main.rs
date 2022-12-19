use std::{
    io::{self, BufRead},
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Dir {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<&Dir> for Point {
    type Output = Point;

    fn add(self, rhs: &Dir) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Cave {
    map: Vec<String>,
    origin: Point,
}

impl Cave {
    fn set(&mut self, p: &Point, c: &str) {
        let _p = *p - self.origin;
        let ux = _p.x as usize;
        let uy = _p.y as usize;
        self.map[uy].replace_range(ux..ux + 1, c);
    }

    // fn at(&self, p: &Point) -> &str {
    //     let _p = *p - self.origin;
    //     let ux = _p.x as usize;
    //     let uy = _p.y as usize;
    //     self.map[uy].get(ux..ux + 1).unwrap()
    // }

    fn is_void_at(&self, p: &Point) -> bool {
        // dbg!(p);
        let _p = *p - self.origin;
        // dbg!(_p);
        if _p.x < 0 {
            // println!("c1: over the left edge");
            return true;
        }
        if _p.x >= self.map[0].len() as i32 {
            // println!("c2: over the right edge");
            return true;
        }
        if _p.y >= self.map.len() as i32 {
            // println!("c3: too low");
            return true;
        }
        // println!("c4: not void");
        false
    }

    fn is_empty_at(&self, p: &Point) -> bool {
        if self.is_void_at(p) {
            return true;
        }

        let _p = *p - self.origin;
        let ux = _p.x as usize;
        let uy = _p.y as usize;
        self.map[uy].get(ux..ux + 1).unwrap() == " "
    }
}

fn parse_input(lines: &[String]) -> Cave {
    let mut max_x = 500;
    let mut max_y = 0;
    let mut min_x = 10000;
    let mut min_y = 0;

    let mut walls: Vec<Vec<Point>> = vec![];

    for line in lines {
        let l = line.replace(' ', "");
        if l.is_empty() {
            continue;
        };
        walls.push(vec![]);
        for pair in line.split(" -> ") {
            let parts: Vec<i32> = pair
                .split(',')
                .map(|n| n.parse::<i32>().expect("a valid number"))
                .collect();
            assert_eq!(parts.len(), 2);
            let x = parts[0];
            let y = parts[1];
            walls.last_mut().unwrap().push(Point { x, y });
            max_x = std::cmp::max(max_x, x);
            max_y = std::cmp::max(max_y, y);
            min_x = std::cmp::min(min_x, x);
            min_y = std::cmp::min(min_y, y);
        }
    }
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut cave = Cave {
        map: vec![" ".repeat(width as usize); height as usize],
        origin: Point { x: min_x, y: min_y },
    };
    for wall in walls {
        for i in 1..wall.len() {
            let p1 = *wall.get(i - 1).unwrap();
            let p2 = *wall.get(i).unwrap();
            if p1.x == p2.x {
                let f = std::cmp::min(p1.y, p2.y);
                let t = std::cmp::max(p1.y, p2.y);
                (f..=t).for_each(|y| {
                    cave.set(&Point { x: p1.x, y }, "#");
                });
            } else if p1.y == p2.y {
                let f = std::cmp::min(p1.x, p2.x);
                let t = std::cmp::max(p1.x, p2.x);
                (f..=t).for_each(|x| {
                    cave.set(&Point { x, y: p1.y }, "#");
                });
            } else {
                panic!("How?!?");
            }
        }
    }
    // cave[0 - min_y].replace_range(500 - min_x..500 - min_x + 1, "X");
    cave
}

fn fall(cave: &mut Cave) -> Option<Point> {
    let mut ball = Point { x: 500, y: 0 };

    let down = Dir { x: 0, y: 1 };
    let downleft = Dir { x: -1, y: 1 };
    let downright = Dir { x: 1, y: 1 };

    // keep increasing y until ball stops or falls to void
    loop {
        // println!("ball: {:?}", &ball);
        let mut moved = false;
        for d in &[down, downleft, downright] {
            let next = ball + d;
            if cave.is_void_at(&next) {
                // println!("cave is void at {:?}", &next);
                return None;
            }
            // else { println!("cave is not void at {:?}", next); }
            if cave.is_empty_at(&next) {
                // println!("cave is empty at {:?}", &next);
                ball = next;
                moved = true;
                break;
            }
            // else { println!("cave is not empty at {:?}", next); }
        }
        // tried 3 directions and it didn't move
        if !moved {
            // println!("ball stopped at: {:?}", ball);
            return Some(ball);
        }
    }
}

fn solution(cave: &mut Cave) -> usize {
    let mut c: usize = 0;
    while let Some(p) = fall(cave) {
        cave.set(&p, ".");
        c += 1;
    }
    for line in &cave.map {
        println!("{}", line);
    }
    c
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
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "24");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "805");
    }
}
