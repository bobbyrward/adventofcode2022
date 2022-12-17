import os
import itertools
import re


STACK_COUNT = 9

def main():
    command_lines = []
    stacks = []

    for i in range(STACK_COUNT):
        stacks.append([])

    with open("../inputs/day05.txt") as fd:
        for line in reversed(list(itertools.takewhile(lambda line: line, (x.strip() for x in fd)))):
            print(f"line: {line}")
            for i in range(STACK_COUNT):
                idx = (i * 4)

                if idx >= len(line) or line[idx] != '[':
                    continue

                stacks[i].append(line[idx + 1])

        for command_line in fd:
            match = re.match(r"^move (?P<count>\d+) from (?P<from>\d) to (?P<to>\d)$", command_line.strip())

            if not match:
                continue

            from_stack = int(match.group("from")) - 1
            to_stack = int(match.group("to")) - 1
            count = int(match.group("count"))

            stacks[to_stack].extend(stacks[from_stack][-count:])
            stacks[from_stack] = stacks[from_stack][:-count]

    answer = ''.join(x[-1] for x in stacks)
    print(f"answer: {answer}")

if __name__ == '__main__':
        main()
