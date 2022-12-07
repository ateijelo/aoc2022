import sys

def solution(lines):
    for line in lines:
        for i in range(0, len(line) - 4):
            s = set(line[i:i+4])
            if len(s) == 4:
                return i + 4
    return 0


if __name__ == "__main__":
    print(solution(line.strip() for line in sys.stdin))


def test_examples():
    assert solution(["bvwbjplbgvbhsrlpgdmjqwftvncz"]) == 5
    assert solution(["nppdvjthqldpwncqszvftbrmjlhg"]) == 6
    assert solution(["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"]) == 10
    assert solution(["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"]) == 11

