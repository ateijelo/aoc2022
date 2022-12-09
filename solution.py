import sys
from dataclasses import dataclass


@dataclass
class Forest:
    trees: list[list[int]]
    width: int
    height: int


def scenic_score(forest: Forest, x: int, y: int):
    v = forest.trees[y][x]

    up = 0
    for ty in range(y - 1, -1, -1):
        h = forest.trees[ty][x]
        up += 1
        if h >= v:
            break

    down = 0
    for ty in range(y + 1, forest.height):
        h = forest.trees[ty][x]
        down += 1
        if h >= v:
            break

    left = 0
    for tx in range(x - 1, -1, -1):
        h = forest.trees[y][tx]
        left += 1
        if h >= v:
            break

    right = 0
    for tx in range(x + 1, forest.width):
        h = forest.trees[y][tx]
        right += 1
        if h >= v:
            break

    score = up * down * left * right
    return score


def solution(lines):
    width = len(lines[0])
    height = len(lines)
    forest = Forest(trees=lines, width=width, height=height)
    score = 0
    for x in range(width):
        for y in range(height):
            score = max(score, scenic_score(forest, x, y))
    return score


if __name__ == "__main__":
    print(solution([line.strip() for line in sys.stdin]))


def test_examples():
    assert (
        solution(
            [
                "30373",
                "25512",
                "65332",
                "33549",
                "35390",
            ]
        )
        == 8
    )
