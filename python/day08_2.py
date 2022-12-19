from typing import List, Optional, Tuple
import itertools

TEST_INPUT="""30373
25512
65332
33549
35390"""


def get_input() -> List[List[int]]:
    with open("../inputs/day08.txt") as fd:
        return [list(map(int, line.strip())) for line in fd.readlines()]

    # return [list(map(int, line)) for line in TEST_INPUT.splitlines()]


def visible_trees(trees: List[int], value: int) -> int:
    seen = 0

    for tree in trees:
        if tree >= value:
            return seen + 1
        seen += 1

    return seen


def score_tree(trees: List[List[int]], x: int, y: int) -> int:
    if x == 0:
        left = 0
    else:
        left = visible_trees(trees[y][x-1::-1], trees[y][x])

    if y == 0:
        top = 0
    else:
        top = visible_trees([row[x] for row in trees[y-1::-1]], trees[y][x])


    if x == len(trees[y]) - 1:
        right = 0
    else:
        right = visible_trees(trees[y][x+1:], trees[y][x])

    if y == len(trees) - 1:
        bottom = 0
    else:
        bottom = visible_trees([row[x] for row in trees[y+1:]], trees[y][x])

    return left * right * top * bottom


in_trees = get_input()

max_scenic = 0

for y in range(len(in_trees)):
    for x in range(len(in_trees[y])):
        score = score_tree(in_trees, x, y)

        if score > max_scenic:
            max_scenic = score

print(f"max_scenic = {max_scenic}")
