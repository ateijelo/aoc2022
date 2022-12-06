import sys
from dataclasses import dataclass

@dataclass
class Range(object):
    start: int
    end: int

    def __init__(self, rangestr: str) -> None:
        a, b = [int(x) for x in rangestr.split("-")]
        self.start = a
        self.end = b

    def contains(self, other):
        return self.start <= other.start and self.end >= other.end

    def overlaps(self, other):
        return self.end >= other.start and self.start <= other.end


def solution(lines):
    count = 0
    for line in lines:
        a, b = line.split(",")
        a, b = Range(a), Range(b)
        if a.contains(b) or b.contains(a):
            count += 1
    return count


if __name__ == "__main__":
    print(solution(line.strip() for line in sys.stdin))


def test_example():
    lines = [
        "2-4,6-8",
        "2-3,4-5",
        "5-7,7-9",
        "2-8,3-7",
        "6-6,4-6",
        "2-6,4-8",
    ]
    assert solution(lines) == 2

def test_range():
    a = Range("5-15")
    assert a.start == 5
    assert a.end == 15

def test_contains():
    a = Range("5-15")
    b = Range("10-25")
    c = Range("1-30")
    assert c.contains(a)
    assert c.contains(b)
    assert not a.contains(b)

def test_overlaps():
    assert Range("5-7").overlaps(Range("7-9"))
    assert Range("2-8").overlaps(Range("3-7"))
    assert Range("6-6").overlaps(Range("4-6"))
    assert Range("2-6").overlaps(Range("4-8"))
    assert Range("2-2").overlaps(Range("2-2"))

    assert not Range("1-1").overlaps(Range("2-2"))
    assert not Range("1-5").overlaps(Range("6-7"))



