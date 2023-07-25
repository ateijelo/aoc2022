use std::{
    char,
    io::{self, BufRead}, collections::HashSet,
};


fn parse_input(lines: &[String]) -> &String {
    &lines[0]
}

fn solution(line: &String) -> usize {
    let marker_length = 14;
    for i in 0..(line.len()-marker_length) {
        let h: HashSet<char> = HashSet::from_iter(line[i..(i+marker_length)].chars());
        if h.len() == marker_length {
            return i + marker_length;
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
        assert_eq!(solution(parse_input(&["mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()])), 19);
        assert_eq!(solution(parse_input(&["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()])), 23);
        assert_eq!(solution(parse_input(&["nppdvjthqldpwncqszvftbrmjlhg".to_string()])), 23);
        assert_eq!(solution(parse_input(&["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()])), 29);
        assert_eq!(solution(parse_input(&["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()])), 26);
    }
}
