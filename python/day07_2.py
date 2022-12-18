from dataclasses import dataclass
import typing


TEST_INPUT = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""


def get_input():
    with open("../inputs/day07.txt") as fd:
        return [x.strip()  for x in fd.read().split("$") if x.strip()]
    # return [x.strip() for x in TEST_INPUT.split("$") if x.strip()]


@dataclass
class Node:
    name: str
    parent: typing.Optional['Node']
    files: typing.Dict[str, int]
    children: typing.Dict[str, 'Node']


    def size(self) -> int:
        return sum(self.files.values()) + sum(x.size() for x in self.children.values())


    def navigate(self, where) -> 'Node':
        if where == "..":
            if self.parent is None:
                raise Exception("node has no parent")

            return self.parent
        else:
            return self.children[where]


    def print(self, indent=0):
        pre = ' ' * indent
        print(f"{pre}- {self.name} (dir) ({self.size()})")
        for file_name, file_size in sorted(self.files.items(), key=lambda x: x[0]):
            filepre = ' ' * (indent + 2)
            print(f"{filepre}- {file_name} (file, size={file_size})")

        for _, child in sorted(self.children.items(), key=lambda x: x[0]):
            child.print(indent+2)


    def filter_by_max_size(self, size: int) -> typing.Iterable['Node']:
        if self.size() < size:
            yield self

        for child in self.children.values():
            yield from child.filter_by_max_size(size)

    def filter_by_min_size(self, size: int) -> typing.Iterable['Node']:
        if self.size() >= size:
            yield self

        for child in self.children.values():
            yield from child.filter_by_min_size(size)


def process_input_chunk(current_node: Node, chunk: str) -> Node:
    # print(f"Chunk: '{chunk}'")
    cmd = chunk[0:2]
    rem = chunk[3:].split("\n")

    if cmd == "ls":
        for line in rem:
            if line[:3] == "dir":
                name = line[4:]
                current_node.children[name] = Node(name=name, parent=current_node, files={}, children={})
            else:
                size, name = line.split(" ")
                current_node.files[name] = int(size)

        return current_node
    elif cmd == "cd":
        assert len(rem) == 1
        return current_node.navigate(rem[0])

    # print(f"cmd: {cmd}\nrem: {rem}")
    # print()

    return current_node


input_str = get_input()
# print(f"'{input_str}'")

input_str = input_str[1:]
root_node = Node(name='/', parent=None, files={}, children={})
current_node = root_node

for chunk in input_str:
    current_node = process_input_chunk(current_node, chunk)

# root_node.print()

TOTAL_SIZE = 70000000
REQUIRED_FREE = 30000000
TARGET_SIZE = TOTAL_SIZE - REQUIRED_FREE

must_free = root_node.size() - TARGET_SIZE

target_node = min(root_node.filter_by_min_size(must_free), key=lambda x: x.size())

print(f"target_node: name={target_node.name}, size={target_node.size()}")

# print(f"free space: {free_space}")

# print(f"< 100000: {sum(x.size() for x in root_node.filter_by_max_size(100000))}")



