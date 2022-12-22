use std::{
    cmp::{max, min},
    collections::HashSet,
    env,
    io::{self, BufRead},
    ops::{Add, Sub},
};

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Dir {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }
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

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn new(from: Point, to: Point) -> Self {
        let w = to.x - from.x;
        let h = to.y - from.y;
        let mut t = to;
        t.x = from.x + i32::signum(w) * min(i32::abs(w), i32::abs(h));
        t.y = from.y + i32::signum(h) * min(i32::abs(w), i32::abs(h));
        Self { from, to: t }
    }

    fn m(&self) -> i32 {
        if self.to.x == self.from.x {
            assert!(self.to.y == self.from.y);
            return 1;
        }
        (self.to.y - self.from.y) / (self.to.x - self.from.x)
    }

    fn n(&self) -> i32 {
        self.from.y - self.m() * self.from.x
    }

    fn from_tuples(from: (i32, i32), to: (i32, i32)) -> Self {
        let from = Point {
            x: from.0,
            y: from.1,
        };
        let to = Point { x: to.0, y: to.1 };
        Self::new(from, to)
    }

    fn crosses(&self, other: &Line) -> bool {
        let ofx_proj = self.m() * other.from.x + self.n();
        let dfx = i32::signum(other.from.y - ofx_proj);

        let otx_proj = self.m() * other.to.x + self.n();
        let dtx = i32::signum(other.to.y - otx_proj);

        if dfx == 0 || dtx == 0 {
            return true;
        }
        dfx != dtx
    }

    fn intersection(&self, other: &Line) -> Option<Line> {
        let sfx = min(self.from.x, self.to.x);
        let stx = max(self.from.x, self.to.x);
        let ofx = min(other.from.x, other.to.x);
        let otx = max(other.from.x, other.to.x);

        if other.m() == self.m() {
            // lines are parallel
            let self_x = vec![(sfx, stx)].to_interval_set();
            let other_x = vec![(ofx, otx)].to_interval_set();
            let x_intersection = self_x.intersection(&other_x);
            if x_intersection.interval_count() == 0 {
                return None;
            }
            if self.n() == other.n() {
                let rfx = x_intersection.lower() as i32;
                let rtx = x_intersection.upper() as i32;
                let rfy = self.m() * rfx + self.n();
                let rty = self.m() * rtx + self.n();
                let rf = Point { x: rfx, y: rfy };
                let rt = Point { x: rtx, y: rty };
                return Some(Line::new(rf, rt));
            }
            return None;
        }

        // if !self.crosses(other) && !other.crosses(self) {
        //     return None;
        // }

        let ix = (other.n() - self.n()) as f64 / (self.m() - other.m()) as f64;
        let iy = self.m() as f64 * ix + self.n() as f64;

        if ix.fract() > 0.25 {
            return None;
        }
        if (sfx as f64) > ix {
            return None;
        };
        if (stx as f64) < ix {
            return None;
        };
        if (ofx as f64) > ix {
            return None;
        };
        if (otx as f64) < ix {
            return None;
        };

        let p = Point {
            x: ix as i32,
            y: iy as i32,
        };
        Some(Line::new(p, p))
    }
}

fn parse_input(lines: &[String]) -> Vec<Sensor> {
    let mut sensors = vec![];
    let re = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let sensor = Sensor {
            position: Point {
                x: caps.name("sx").unwrap().as_str().parse().unwrap(),
                y: caps.name("sy").unwrap().as_str().parse().unwrap(),
            },
            beacon: Point {
                x: caps.name("bx").unwrap().as_str().parse().unwrap(),
                y: caps.name("by").unwrap().as_str().parse().unwrap(),
            },
        };
        sensors.push(sensor);
    }
    sensors
}

use gcollections::ops::*;
use interval::interval_set::*;

fn solution(sensors: Vec<Sensor>, row: i32) -> usize {
    let mut coverage = 0;
    let target_row = row;
    let mut beacons_on_row = HashSet::new();
    let mut cover = vec![].to_interval_set();
    for sensor in sensors {
        let beacon_distance = sensor.position.manhattan_distance(&sensor.beacon);
        let row_distance = i32::abs(sensor.position.y - target_row);
        // println!(
        //     "{:?}, beacon_distance={}, row_distance={}",
        //     sensor, beacon_distance, row_distance
        // );
        if row_distance <= beacon_distance {
            let interval = vec![(
                sensor.position.x - beacon_distance + row_distance,
                sensor.position.x + beacon_distance - row_distance,
            )]
            .to_interval_set();

            println!("adding interval: {}", interval);
            cover = cover.union(&interval);
        }
        if sensor.beacon.y == target_row {
            beacons_on_row.insert(sensor.beacon.x);
        }
    }
    println!("result:");
    println!("intervals: {}", cover);
    println!("beacons on row: {:?}", beacons_on_row);
    for i in cover.iter() {
        coverage += i.upper() - i.lower() + 1;
    }
    coverage as usize - beacons_on_row.len()
}

fn solve(lines: &[String], row: i32) -> usize {
    solution(parse_input(lines), row)
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let args: Vec<String> = env::args().collect();
    println!("{}", solve(&lines, args[1].parse().unwrap()));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn test_file(filename: &str, row: i32, solution: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines, row).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", 10, "26");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", 2000000, "5870800");
    }

    #[test]
    fn test_line() {
        let l = Line::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        assert_eq!(l.to, Point { x: 10, y: 10 });

        let l = Line::new(Point { x: 0, y: 0 }, Point { x: 5, y: 10 });
        assert_eq!(l.to, Point { x: 5, y: 5 });

        let l = Line::new(Point { x: 0, y: 0 }, Point { x: -5, y: 10 });
        assert_eq!(l.to, Point { x: -5, y: 5 });

        let l = Line::new(Point { x: 0, y: 0 }, Point { x: -5, y: -5 });
        assert_eq!(l.to, Point { x: -5, y: -5 });

        let l = Line::new(Point { x: 0, y: 0 }, Point { x: -10, y: -5 });
        assert_eq!(l.to, Point { x: -5, y: -5 });
    }

    #[test]
    fn test_intersection() {
        let l1 = Line::from_tuples((0, 0), (10, 10));
        let l2 = Line::from_tuples((5, 5), (15, 15));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((5, 5), (10, 10))
        );

        let l1 = Line::from_tuples((0, 0), (10, 10));
        let l2 = Line::from_tuples((5, 5), (5, 5));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((5, 5), (5, 5))
        );

        let l1 = Line::new(Point { x: 0, y: 0 }, Point { x: 5, y: 5 });
        let l2 = Line::new(Point { x: 6, y: 6 }, Point { x: 10, y: 10 });
        assert_eq!(l1.intersection(&l2), None);

        let l1 = Line::from_tuples((0, 0), (5, 5));
        let l2 = Line::from_tuples((5, 5), (10, 10));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((5, 5), (5, 5))
        );

        let l1 = Line::from_tuples((0, 0), (5, -5));
        let l2 = Line::from_tuples((5, -5), (10, -10));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((5, -5), (5, -5))
        );

        let l1 = Line::from_tuples((-2, 2), (10, -10));
        let l2 = Line::from_tuples((-10, 10), (2, -2));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((-2, 2), (2, -2))
        );

        let l1 = Line::from_tuples((-2, 2), (10, -10));
        let l2 = Line::from_tuples((-9, 10), (3, -2));
        assert_eq!(l1.intersection(&l2), None);

        let l1 = Line::new(Point { x: 10, y: 10 }, Point { x: 5, y: 5 });
        let l2 = Line::new(Point { x: 5, y: 5 }, Point { x: 0, y: 0 });
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((5, 5), (5, 5))
        );

        let l1 = Line::from_tuples((0, 0), (5, 5));
        let l2 = Line::from_tuples((5, 0), (0, 5));
        assert_eq!(l1.intersection(&l2), None);

        let l1 = Line::from_tuples((0, 0), (5, 5));
        let l2 = Line::from_tuples((4, 0), (0, 4));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((2, 2), (2, 2))
        );

        let l1 = Line::from_tuples((2, 0), (12, 10));
        let l2 = Line::from_tuples((2, 0), (8, -6));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((2, 0), (2, 0))
        );

        let l1 = Line::from_tuples((0, 0), (0, 0));
        let l2 = Line::from_tuples((0, 0), (0, 0));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((0, 0), (0, 0))
        );

        let l1 = Line::from_tuples((0, 0), (0, 0));
        let l2 = Line::from_tuples((-5, -5), (5, 5));
        assert_eq!(
            l1.intersection(&l2).unwrap(),
            Line::from_tuples((0, 0), (0, 0))
        );
    }

    #[test]
    fn test_crosses() {
        let l1 = Line::from_tuples((0, 0), (5, 5));
        let l2 = Line::from_tuples((5, 0), (0, 5));
        assert!(l1.crosses(&l2));

        let l1 = Line::from_tuples((0, 0), (4, 4));
        let l2 = Line::from_tuples((4, 0), (0, 4));
        assert!(l1.crosses(&l2));

        let l1 = Line::from_tuples((0, 0), (4, 4));
        let l2 = Line::from_tuples((2, 3), (0, 5));
        assert!(!l1.crosses(&l2));
    }
}
