import sys

def solution(lines):
    for line in lines:
        for i in range(0, len(line) - 14):
            s = set(line[i:i+14])
            if len(s) == 14:
                return i + 14
    return 0


if __name__ == "__main__":
    print(solution(line.strip() for line in sys.stdin))


def test_examples():
    assert solution(["mjqjpqmgbljsphdztnvjfqwrcgsmlb"]) == 19
    assert solution(["bvwbjplbgvbhsrlpgdmjqwftvncz"]) == 23
    assert solution(["nppdvjthqldpwncqszvftbrmjlhg"]) == 23
    assert solution(["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"]) == 29
    assert solution(["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"]) == 26

