use std::io::{self, BufRead};

fn parse_input(lines: &[String]) -> List {
    let mut r = List::new();

    for (i, line) in lines.iter().enumerate() {
        r.push((line.parse().unwrap(), i));
    }
    r
}

type List = Vec<(i64, usize)>;

trait CircularList {
    fn move_item(&mut self, from: usize, to: usize);
    fn move_circular(&mut self, pos: usize, count: &i64);
    fn print_from_zero(&self);
}

impl CircularList for List {

    fn move_item(&mut self, from: usize, to: usize) {
        assert!(to < self.len());
        assert!(from < self.len());
        if to == from {
            return;
        }
        let elem = self[from];
        let store_ptr = self.as_mut_ptr();
        if to > from {
            unsafe {
                let src = store_ptr.add(from + 1);
                let dst = store_ptr.add(from);
                std::ptr::copy(src, dst, to - from);
            }
        } else {
            unsafe {
                let src = store_ptr.add(to);
                let dst = store_ptr.add(to + 1);
                std::ptr::copy(src, dst, from - to);
            }
        }
        self[to] = elem;
    }

    fn move_circular(&mut self, pos: usize, count: &i64) {
        let count = count.rem_euclid(self.len() as i64 - 1);
        if count == 0 {
            return;
        }
        let to = (pos as i64 + count).rem_euclid(self.len() as i64 - 1) as usize;
        self.move_item(pos, to);
    }

    fn print_from_zero(&self) {
        let zp = self.iter().position(|x| x.0 == 0).unwrap();
        for i in 0..self.len() {
            print!(
                "{}{}",
                self[(zp + i).rem_euclid(self.len())].0,
                if i < (self.len() - 1) { ", " } else { "" }
            );
        }
        println!();
    }
}

fn solution(nums: &mut List) -> i64 {
    let orig = nums.clone();
    for value in orig.iter() {
        let from = nums.iter().position(|x| x == value).unwrap();
        nums.move_circular(from, &value.0);
    }
    let zp = nums.iter().position(|x| x.0 == 0).unwrap();
    let a = nums[(zp + 1000).rem_euclid(nums.len())];
    let b = nums[(zp + 2000).rem_euclid(nums.len())];
    let c = nums[(zp + 3000).rem_euclid(nums.len())];
    a.0 + b.0 + c.0
}

fn solve(lines: &[String]) -> i64 {
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
        test_file("example.txt", "3");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "8028");
    }

    fn make_list(vec: Vec<i64>) -> List {
        vec.iter().map(|val| (*val, *val as usize)).collect()
    }

    #[test]
    fn test_move_circular() {
        let mut list = make_list(vec![1, 2, 3, 4, 5, 6]);
        let expected = make_list(vec![2, 1, 3, 4, 5, 6]);
        list.move_circular(0, &1);
        assert_eq!(list, expected);

        let mut list = make_list(vec![1, 2, 3, 4, 5, 6]);
        let expected = make_list(vec![2, 3, 4, 5, 1, 6]);
        list.move_circular(0, &-1);
        assert_eq!(list, expected);

        let mut list = make_list(vec![1, 2, 3, 4, 5, 6]);
        let expected = make_list(vec![1, 2, 3, 4, 5, 6]);
        list.move_circular(0, &10);
        assert_eq!(list, expected);
        list.move_circular(0, &15);
        assert_eq!(list, expected);
        list.move_circular(1, &15);
        assert_eq!(list, expected);
        list.move_circular(5, &15);
        assert_eq!(list, expected);
    }
}
