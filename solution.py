import sys
from dataclasses import dataclass


@dataclass
class Forest:
    trees: list[list[int]]
    width: int
    height: int


def is_visible(forest: Forest, x: int, y: int):
    v = forest.trees[y][x]

    up = [forest.trees[ty][x] for ty in range(y)]
    down = [forest.trees[ty][x] for ty in range(y + 1, forest.height)]
    right = [forest.trees[y][tx] for tx in range(x + 1, forest.width)]
    left = [forest.trees[y][tx] for tx in range(x)]

    return any(
        [
            all(h < v for h in up),
            all(h < v for h in right),
            all(h < v for h in down),
            all(h < v for h in left),
        ]
    )


def solution(lines):
    t = 0
    width = len(lines[0])
    height = len(lines)
    forest = Forest(trees=lines, width=width, height=height)
    for x in range(width):
        for y in range(height):
            if is_visible(forest, x, y):
                t += 1
    return t


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
        == 21
    )
