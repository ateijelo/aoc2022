use std::io;

#[derive(Debug, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    elf: Play,
    outcome: Outcome,
}

fn map_play(s: &str) -> Play {
    match s {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!(),
    }
}

fn map_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!(),
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Round> {
    let mut rounds = vec![];
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        rounds.push(Round {
            elf: map_play(words[0]),
            outcome: map_outcome(words[1]),
        });
    }
    rounds
}

// A rock, B paper, C scissors
// X rock, Y paper, Z scissors

fn play(round: &Round) -> i32 {
    // rock beats scissors
    // scissors beat paper
    // paper beats rock

    let my_play = match (&round.elf, &round.outcome) {
        (Play::Rock, Outcome::Loss) => Play::Scissors,
        (Play::Rock, Outcome::Draw) => Play::Rock,
        (Play::Rock, Outcome::Win) => Play::Paper,
        (Play::Paper, Outcome::Loss) => Play::Rock,
        (Play::Paper, Outcome::Draw) => Play::Paper,
        (Play::Paper, Outcome::Win) => Play::Scissors,
        (Play::Scissors, Outcome::Loss) => Play::Paper,
        (Play::Scissors, Outcome::Draw) => Play::Scissors,
        (Play::Scissors, Outcome::Win) => Play::Rock,
    };

    println!("playing round: {:?}", round);
    let selection_score = match my_play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    println!("selection score: {:?}", selection_score);
    let outcome_score = match (&round.elf, &my_play) {
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
        score += play(&round);
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
                    outcome: Outcome::Draw
                },
                Round {
                    elf: Play::Paper,
                    outcome: Outcome::Loss
                },
                Round {
                    elf: Play::Scissors,
                    outcome: Outcome::Win
                }
            ]
        );
    }

    #[test]
    fn test_solution() {
        let rounds = vec![
            Round {
                elf: Play::Rock,
                outcome: Outcome::Draw,
            },
            Round {
                elf: Play::Paper,
                outcome: Outcome::Loss,
            },
            Round {
                elf: Play::Scissors,
                outcome: Outcome::Win,
            },
        ];
        assert_eq!(solution(rounds), 12);
    }
}
