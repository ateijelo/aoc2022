use std::io::{self, BufRead};

fn parse_input(lines: &[String]) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = vec![];
    for line in lines {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
    map
}

fn is_visible(map: &[Vec<u32>], x: usize, y: usize) -> bool {
    let v = map[y][x];
    let height = map.len();
    let width = map[0].len();

    let mut up = true;
    for row in map.iter().take(y) {
        if row[x] >= v {
            up = false;
            break;
        }
    }

    let mut down = true;
    // for ty in y+1..height {
    for row in map.iter().take(height).skip(y + 1) {
        if row[x] >= v {
            down = false;
            break;
        }
    }

    let mut left = true;
    for tx in 0..x {
        if map[y][tx] >= v {
            left = false;
            break;
        }
    }

    let mut right = true;
    for tx in x + 1..width {
        if map[y][tx] >= v {
            right = false;
            break;
        }
    }

    up || down || left || right
}

fn solution(map: &Vec<Vec<u32>>) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if is_visible(map, x, y) {
                count += 1;
            }
        }
    }
    count
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
        assert_eq!(solution(&map), 21);
    }
}
