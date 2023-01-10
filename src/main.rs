use std::{
    collections::HashSet,
    fmt::Debug,
    io::{self, BufRead},
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

fn parse_input(lines: &[String]) -> Vec<Point3D> {
    let mut r = Vec::new();
    for line in lines {
        let Some((x, y, z)) = line.split(',').map(|s| s.parse::<i32>().unwrap()).tuples().next() else { continue; };
        r.push(Point3D { x, y, z });
    }
    r
}

fn solution(points: &[Point3D]) -> u32 {
    let mut sides = HashSet::new();

    for p in points.iter() {
        // each cube provides 6 sides
        // println!("p: {:?}", p);
        let cube_sides = [
            Point3D::new(2 * p.x + 1, 2 * p.y + 1, 2 * p.z),
            Point3D::new(2 * p.x + 1, 2 * p.y, 2 * p.z + 1),
            Point3D::new(2 * p.x, 2 * p.y + 1, 2 * p.z + 1),
            Point3D::new(2 * p.x + 1, 2 * p.y + 1, 2 * p.z + 2),
            Point3D::new(2 * p.x + 1, 2 * p.y + 2, 2 * p.z + 1),
            Point3D::new(2 * p.x + 2, 2 * p.y + 1, 2 * p.z + 1),
        ];
        for cube_side in cube_sides {
            // println!("   cube_side: {:?}", cube_side);
            if sides.contains(&cube_side) {
                sides.remove(&cube_side);
            } else {
                sides.insert(cube_side);
            }
        }
    }
    sides.len() as u32
}

fn solve(lines: &[String]) -> u32 {
    // let c: usize = env::args().collect::<Vec<String>>()[1].parse().unwrap();
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

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "64");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "3396");
    }
}
