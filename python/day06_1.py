

def get_input():
    with open("../inputs/day06.txt") as fd:
        return fd.read().strip()


buffer = get_input()

for idx, (x, y, z, w) in enumerate(zip(buffer, buffer[1:], buffer[2:], buffer[3:])):
    if len(set((x, y, z, w))) == 4:
        print(idx + 4)
        break
