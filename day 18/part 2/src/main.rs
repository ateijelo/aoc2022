use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    fmt::Debug,
    io::{self, BufRead},
    ops::Add,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Copy for Point3D {}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn between(&self, a: &Point3D, b: &Point3D) -> bool {
        if self.x < min(a.x, b.x) {
            return false;
        }
        if self.x > max(a.x, b.x) {
            return false;
        }
        if self.y < min(a.y, b.y) {
            return false;
        }
        if self.y > max(a.y, b.y) {
            return false;
        }
        if self.z < min(a.z, b.z) {
            return false;
        }
        if self.z > max(a.z, b.z) {
            return false;
        }
        true
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
    let mut point_set: HashSet<Point3D> = HashSet::new();
    for p in points.iter() {
        point_set.insert(*p);
    }

    let mut pmin = *points.first().unwrap();
    let mut pmax = *points.first().unwrap();

    for p in point_set.iter() {
        pmin.x = std::cmp::min(pmin.x, p.x - 1); // use -1 here to guarantee minimum is always outside
        pmin.y = std::cmp::min(pmin.y, p.y - 1);
        pmin.z = std::cmp::min(pmin.z, p.z - 1);
        pmax.x = std::cmp::max(pmax.x, p.x + 1);
        pmax.y = std::cmp::max(pmax.y, p.y + 1);
        pmax.z = std::cmp::max(pmax.z, p.z + 1);
    }

    let mut q: VecDeque<Point3D> = VecDeque::new();

    let directions = [
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
    ];
    
    // now BFS from one corner
    let mut count = 0;
    q.push_back(pmin);
    let mut visited = HashSet::new();
    visited.insert(pmin);
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for (dx, dy, dz) in directions {
            let d = Point3D::new(dx, dy, dz);
            if !(p + d).between(&pmin, &pmax) {
                continue;
            }
            if point_set.contains(&(p + d)) {
                // every block we run into is from one side, so count it
                count += 1;
                continue;
            }
            if visited.contains(&(p + d)) {
                continue;
            }
            visited.insert(p + d);
            q.push_back(p + d);
        }
    }

    count
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
        test_file("example.txt", "58");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "2044");
    }
}
