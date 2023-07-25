use std::{
    collections::{HashMap, HashSet},
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

    fn can_move_in_direction(&self, positions: &HashSet<Point>, d: &Point) -> bool {
        let mut a = *self + Point { x: -1, y: d.y };
        let b = self + d;
        let mut c = *self + Point { x: 1, y: d.y };
        if d.y == 0 {
            a = *self + Point { x: d.x, y: -1 };
            c = *self + Point { x: d.x, y: 1 };
        }
        !(positions.contains(&a) || positions.contains(&b) || positions.contains(&c))
    }

    fn is_alone(&self, positions: &HashSet<Point>) -> bool {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if positions.contains(&(self + &Point::new(dx, dy))) {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_input(lines: &[String]) -> Vec<Point> {
    let mut r = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                r.push(Point {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }
    r
}

// fn print_positions(positions: &HashSet<Point>) {
//     println!("(-3, -2)");
//     for y in -2..=9 {
//         for x in -3..=10 {
//             if positions.contains(&Point { x, y }) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!()
//     }
// }

fn solution(points: &[Point]) -> u32 {
    let mut positions: HashSet<Point> = points.iter().copied().collect();
    // key is "destination", value is "source"
    let mut new_positions: HashMap<Point, Vec<Point>> = HashMap::new();

    let directions = [
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(1, 0),
    ];
    let mut next_dir = directions.iter().cycle();

    for _ in 0..10 {
        let mut dir = next_dir.next().unwrap();
        for p in positions.iter() {
            if p.is_alone(&positions) {
                continue;
            }
            for i in 0..4 {
                if p.can_move_in_direction(&positions, dir) {
                    let dest = p + dir;
                    let v = new_positions.entry(dest).or_default();
                    v.push(*p);
                    for _ in 0..(4 - i) {
                        dir = next_dir.next().unwrap();
                    }
                    break;
                }
                dir = next_dir.next().unwrap();
            }
        }
        let mut someone_moved = false;
        for (dest, sources) in new_positions.iter() {
            if sources.len() == 1 {
                let source = sources.first().unwrap();
                if source != dest {
                    // make the move
                    positions.remove(source);
                    positions.insert(*dest);
                    someone_moved = true;
                }
            }
        }
        new_positions.clear();
        if !someone_moved {
            break;
        }
    }

    let minx = positions.iter().map(|p| p.x).min().unwrap();
    let miny = positions.iter().map(|p| p.y).min().unwrap();
    let maxx = positions.iter().map(|p| p.x).max().unwrap();
    let maxy = positions.iter().map(|p| p.y).max().unwrap();

    (maxx.abs_diff(minx) + 1) * (maxy.abs_diff(miny) + 1) - positions.len() as u32
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
        test_file("example.txt", "110");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "3925");
    }
}
