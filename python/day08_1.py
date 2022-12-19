from typing import List

TEST_INPUT="""30373
25512
65332
33549
35390"""


def get_input() -> List[List[int]]:
    with open("../inputs/day08.txt") as fd:
        return [list(map(int, line.strip())) for line in fd.readlines()]

    # return [list(map(int, line)) for line in TEST_INPUT.splitlines()]


def is_visible(trees: List[List[int]], x: int, y: int) -> bool:
    value = trees[y][x]

    if x == 0 or y == 0 or x == len(trees[y]) - 1 or y == len(trees) - 1:
        return True

    found = True
    for cx in range(0, x):
        if value <= trees[y][cx]:
            found = False
            break

    if found:
        # print(f"({x}, {y})={value} is visible from the left")
        return True

    found = True
    for cx in range(x+1, len(trees[y])):
        if value <= trees[y][cx]:
            found = False
            break

    if found:
        # print(f"({x}, {y})={value} is visible from the right")
        return True

    found = True
    for cy in range(0, y):
        if value <= trees[cy][x]:
            found = False
            break

    if found:
        # print(f"({x}, {y})={value} is visible from the top")
        return True

    found = True
    for cy in range(y+1, len(trees)):
        if value <= trees[cy][x]:
            found = False
            break

    if found:
        # print(f"({x}, {y})={value} is visible from the bottom")
        return True

    return False


in_trees = get_input()

count = 0

for y in range(len(in_trees)):
    for x in range(len(in_trees[y])):
        if is_visible(in_trees, x, y):
            # print(f"is_visible: ({x}, {y})")
            count += 1

print(f"total = {count}")

