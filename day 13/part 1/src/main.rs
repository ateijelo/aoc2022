use std::io::{self, BufRead};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct PacketPair(Item, Item);

fn parse_input(lines: &[String]) -> Vec<PacketPair> {
    let mut result = vec![];
    let mut p = PacketPair(Item::Number(0), Item::Number(0));
    for (i, line) in lines.iter().enumerate() {
        let l = line.replace(' ', "");
        if i % 3 == 0 {
            p.0 = list(&l).unwrap().1;
        }
        if i % 3 == 1 {
            p.1 = list(&l).unwrap().1;
        }
        if i % 3 == 2 {
            result.push(p);
            p = PacketPair(Item::Number(0), Item::Number(0));
        }

    }
    result.push(p);
    result
}

fn compare_nums(left: &u32, right: &u32) -> i32 {
    if left < right {
        return 1;
    };
    if right < left {
        return -1;
    };
    0
}

fn compare_lists(left: &Vec<Item>, right: &Vec<Item>) -> i32 {
    // first, compare each item
    for i in 0..std::cmp::min(left.len(), right.len()) {
        let c = compare_items(&left[i], &right[i]);
        if c != 0 {
            return c;
        }
    }
    // if we reached this point, we compare list lengths
    if left.len() < right.len() { return 1; }
    if left.len() > right.len() { return -1; }
    0
}

fn compare_items(left: &Item, right: &Item) -> i32 {
    match left {
        Item::Number(l_num) => {
            match right {
                Item::Number(r_num) => {
                    compare_nums(l_num, r_num)
                }
                Item::List(r_list) => {
                    let l_list = vec![left.clone()];
                    compare_lists(&l_list, r_list)
                }
            }
        }
        Item::List(l_list) => {
            match right {
                Item::Number(_) => {
                    let r_list = vec![right.clone()];
                    compare_lists(l_list, &r_list)
                }
                Item::List(r_list) => {
                    compare_lists(l_list, r_list)
                }
            }
        }
    }
}

fn compare_pairs(pair: &PacketPair) -> i32 {
    compare_items(&pair.0, &pair.1)
}

fn solution(pairs: Vec<PacketPair>) -> usize {
    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if compare_pairs(pair) > 0 {
            // pair index starts at 1
            sum += i + 1;
        }
    }
    sum
}

fn solve(lines: &[String]) -> usize {
    solution(parse_input(lines))
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    // for line in solve(&lines) {
    //     println!("{}", line);
    // }
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn test_example() {
        let reader = BufReader::new(File::open("example.txt").unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .collect();
        assert_eq!(solve(&lines), 13);
    }
}
