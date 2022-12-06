use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    elf: Play,
    me: Play,
}

fn map(s: &str) -> Play {
    match s {
        "A" | "X" => Play::Rock,
        "B" | "Y" => Play::Paper,
        "C" | "Z" => Play::Scissors,
        _ => panic!(),
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Round> {
    let mut rounds = vec![];
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        rounds.push(Round {
            elf: map(words[0]),
            me: map(words[1]),
        });
    }
    rounds
}

// A rock, B paper, C scissors
// X rock, Y paper, Z scissors

fn play(round: Round) -> i32 {
    // rock beats scissors
    // scissors beat paper
    // paper beats rock
    println!("playing round: {:?}", round);
    let selection_score = match round.me {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    println!("selection score: {:?}", selection_score);
    let outcome_score = match (round.elf, round.me) {
        (Play::Rock, Play::Rock) => 3,
        (Play::Rock, Play::Paper) => 6,
        (Play::Rock, Play::Scissors) => 0,
        (Play::Paper, Play::Rock) => 0,
        (Play::Paper, Play::Paper) => 3,
        (Play::Paper, Play::Scissors) => 6,
        (Play::Scissors, Play::Rock) => 6,
        (Play::Scissors, Play::Paper) => 0,
        (Play::Scissors, Play::Scissors) => 3,
    };
    println!("outcome score: {:?}", outcome_score);
    selection_score + outcome_score
}

fn solution(rounds: Vec<Round>) -> i32 {
    let mut score = 0;
    for round in rounds {
        score += play(round);
    }
    score
}

fn main() {
    println!(
        "{:?}",
        solution(parse_input(
            io::stdin().lines().map(|line| line.unwrap()).collect()
        ))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = vec!["A Y", "B X", "C Z"];
        let rounds = parse_input(lines.iter().map(|x| x.to_string()).collect());
        assert_eq!(
            rounds,
            vec![
                Round {
                    elf: Play::Rock,
                    me: Play::Paper
                },
                Round {
                    elf: Play::Paper,
                    me: Play::Rock
                },
                Round {
                    elf: Play::Scissors,
                    me: Play::Scissors
                }
            ]
        );
    }

    #[test]
    fn test_solution() {
        let rounds = vec![
            Round {
                elf: Play::Rock,
                me: Play::Paper,
            },
            Round {
                elf: Play::Paper,
                me: Play::Rock,
            },
            Round {
                elf: Play::Scissors,
                me: Play::Scissors,
            },
        ];
        assert_eq!(solution(rounds), 15);
    }
}
