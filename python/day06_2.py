

def get_input():
    with open("../inputs/day06.txt") as fd:
        return fd.read().strip()


buffer = get_input()

for idx in range(len(buffer) - 14):
    if len(set(buffer[idx:idx+14])) == 14:
        print(idx+14)
        break
