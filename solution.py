import sys

# (Play::Rock, Outcome::Loss) => Play::Scissors,
# (Play::Rock, Outcome::Draw) => Play::Rock,
# (Play::Rock, Outcome::Win) => Play::Paper,
# (Play::Paper, Outcome::Loss) => Play::Rock,
# (Play::Paper, Outcome::Draw) => Play::Paper,
# (Play::Paper, Outcome::Win) => Play::Scissors,
# (Play::Scissors, Outcome::Loss) => Play::Paper,
# (Play::Scissors, Outcome::Draw) => Play::Scissors,
# (Play::Scissors, Outcome::Win) => Play::Rock,

cases = {
    "A X": 3 + 0,
    "A Y": 1 + 3,
    "A Z": 2 + 6,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 2 + 0,
    "C Y": 3 + 3,
    "C Z": 1 + 6,
}

score = 0
for line in sys.stdin:
    score += cases[line.strip()]

print(score)
