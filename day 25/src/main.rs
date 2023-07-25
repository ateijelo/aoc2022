use std::io::{self, BufRead};

fn parse_input(lines: &[String]) -> Vec<String> {
    lines.to_vec()
}

fn snafu_to_i64(s: &str) -> i64 {
    let mut power = 1;
    let mut sum = 0;
    for c in s.chars().rev() {
        match c {
            '=' => sum += -2 * power,
            '-' => sum -= power,
            '0' => sum += 0,
            '1' => sum += power,
            '2' => sum += 2 * power,
            _ => panic!(),
        }
        power *= 5;
    }
    sum
}

fn i64_to_snafu(n: i64) -> String {
    let mut d = 0;
    loop {
        let p = 5i64.pow(d);
        let offset = (p - 1) / 2;
        let bucket = (n + offset) / p;
        if bucket.abs() <= 2 {
            break;
        }
        d += 1;
    }
    // println!("now backwards from d={}", d);
    let mut n = n;
    let mut r = "".to_owned();
    loop {
        let p = 5i64.pow(d);
        let offset = (p - 1) / 2;
        let bucket = num::integer::div_floor(n + offset, p);
        n -= bucket * p;

        r.push(match bucket {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        });

        if d == 0 {
            break;
        }
        d -= 1;
    }
    r
}

fn solution(lines: &[String]) -> String {
    let mut sum = 0;
    for line in lines {
        let n = snafu_to_i64(line);
        println!("{}", n);
        sum += n;
    }
    println!("{}", sum);
    i64_to_snafu(sum)
}

fn solve(lines: &[String]) -> String {
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
        assert_eq!(solve(&lines), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "2=-1=0");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "2=10---0===-1--01-20");
    }

    #[test]
    fn test_i64_to_snafu() {
        assert_eq!(i64_to_snafu(15), "1=0");
        assert_eq!(i64_to_snafu(20), "1-0");
        assert_eq!(i64_to_snafu(2022), "1=11-2");
        assert_eq!(i64_to_snafu(12345), "1-0---0");
        assert_eq!(i64_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_snafu_to_i64() {
        assert_eq!(snafu_to_i64("1=-0-2"), 1747);
        assert_eq!(snafu_to_i64("12111"), 906);
        assert_eq!(snafu_to_i64("2=0="), 198);
        assert_eq!(snafu_to_i64("21"), 11);
        assert_eq!(snafu_to_i64("2=01"), 201);
        assert_eq!(snafu_to_i64("111"), 31);
    }
}
