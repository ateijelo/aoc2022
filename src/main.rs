use std::{
    char,
    io::{self, BufRead}, collections::HashSet,
};


fn parse_input(lines: &[String]) -> &String {
    &lines[0]
}

fn solution(line: &String) -> usize {
    for i in 0..(line.len()-4) {
        let h: HashSet<char> = HashSet::from_iter(line[i..(i+4)].chars());
        if h.len() == 4 {
            return i + 4;
        }
    }
    0
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solution(parse_input(&lines)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solution(parse_input(&["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()])), 5);
        assert_eq!(solution(parse_input(&["nppdvjthqldpwncqszvftbrmjlhg".to_string()])), 6);
        assert_eq!(solution(parse_input(&["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()])), 10);
        assert_eq!(solution(parse_input(&["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()])), 11);
    }
}
