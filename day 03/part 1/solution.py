import sys


def value(letter):
    if "a" <= letter <= "z":
        return ord(letter) - ord("a") + 1
    if "A" <= letter <= "Z":
        return ord(letter) - ord("A") + 27
    return 0


def solution(lines):
    sum = 0
    for line in lines:
        l = len(line)
        a = line[: l // 2]
        b = line[l // 2 :]
        common = set(a).intersection(set(b))
        # print(a, b, value(common.pop()))
        sum += value(common.pop())
    return sum


if __name__ == "__main__":
    print(solution(line.strip() for line in sys.stdin))


def test_value():
    assert value("a") == 1
    assert value("z") == 26
    assert value("A") == 27
    assert value("Z") == 52


def test_example():
    lines = [
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ]
    assert solution(lines) == 157
