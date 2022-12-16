use std::cmp::Ordering;
use std::io::{self, BufRead};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

fn number(input: &str) -> IResult<&str, Item> {
    let result = u32(input);
    result.map(|(input, num)| (input, Item::Number(num)))
}

fn list(input: &str) -> IResult<&str, Item> {
    let result = delimited(tag("["), items, tag("]"))(input);
    result.map(|(input, items)| (input, Item::List(items)))
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((number, list))(input)
}

fn items(input: &str) -> IResult<&str, Vec<Item>> {
    separated_list0(tag(","), item)(input)
}

fn parse_input(lines: &[String]) -> Vec<Item> {
    let mut result = vec![];
    for line in lines {
        let l = line.replace(' ', "");
        if l.is_empty() {
            continue;
        };
        result.push(list(&l).unwrap().1);
    }
    result
}

fn compare_nums(left: &u32, right: &u32) -> Ordering {
    if left < right {
        return Ordering::Less;
    };
    if right < left {
        return Ordering::Greater;
    };
    Ordering::Equal
}

fn compare_lists(left: &Vec<Item>, right: &Vec<Item>) -> Ordering {
    // first, compare each item
    for i in 0..std::cmp::min(left.len(), right.len()) {
        let c = compare_items(&left[i], &right[i]);
        if c != Ordering::Equal {
            return c;
        }
    }
    // if we reached this point, we compare list lengths
    if left.len() < right.len() {
        return Ordering::Less;
    }
    if left.len() > right.len() {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn compare_items(left: &Item, right: &Item) -> Ordering {
    match left {
        Item::Number(l_num) => match right {
            Item::Number(r_num) => compare_nums(l_num, r_num),
            Item::List(r_list) => {
                let l_list = vec![left.clone()];
                compare_lists(&l_list, r_list)
            }
        },
        Item::List(l_list) => match right {
            Item::Number(_) => {
                let r_list = vec![right.clone()];
                compare_lists(l_list, &r_list)
            }
            Item::List(r_list) => compare_lists(l_list, r_list),
        },
    }
}

fn solution(mut items: Vec<Item>) -> usize {
    let div2 = list("[[2]]").unwrap().1;
    let div6 = list("[[6]]").unwrap().1;
    items.push(div2);
    items.push(div6);
    items.sort_by(compare_items);

    let div2 = list("[[2]]").unwrap().1;
    let div6 = list("[[6]]").unwrap().1;
    let mut result = 1;
    for (i, item) in items.iter().enumerate() {
        if *item == div2 || *item == div6 {
            result *= i + 1;
        }
    }
    result
}

fn solve(lines: &[String]) -> usize {
    solution(parse_input(lines))
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
        test_file("example.txt", "140");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "24805");
    }
}
