use std::io::{self, BufRead};

fn parse_input(lines: &[String]) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];
    for line in lines {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
    map
}

fn scenic_score(map: &[Vec<u32>], x: usize, y: usize) -> usize {
    let v = map[y][x];
    // let height = map.len();
    let width = map[0].len();

    let mut up = 0;
    for ty in (0..y).rev() {
        up += 1;
        if map[ty][x] >= v {
            break;
        }
    }

    let mut down = 0;
    for row in map.iter().skip(y + 1) {
        down += 1;
        if row[x] >= v {
            break;
        }
    }

    let mut left = 0;
    for tx in (0..x).rev() {
        left += 1;
        if map[y][tx] >= v {
            break;
        }
    }

    let mut right = 0;
    for tx in x + 1..width {
        right += 1;
        if map[y][tx] >= v {
            break;
        }
    }

    up * down * left * right
}

fn solution(map: &Vec<Vec<u32>>) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            score = std::cmp::max(score, scenic_score(map, x, y));
        }
    }
    score
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solution(&parse_input(&lines)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = "
            30373
            25512
            65332
            33549
            35390
        ";

        let lines: Vec<String> = lines
            .lines()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        let map = parse_input(&lines);
        assert_eq!(solution(&map), 8);
    }
}
