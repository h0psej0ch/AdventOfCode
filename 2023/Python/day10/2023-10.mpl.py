import matplotlib.pyplot as plt
from matplotlib.path import Path

data = open('data10.txt').read().split('\n')

for i in range(len(data)):
    found = data[i].find('S')
    if found != -1:
        print(i)
        print(found)
        break

cur = "|"
mov = (0, -1)
pos = (found, i)

def pathfinder(data, pos, mov):
    path = []
    while True:
        pos = (pos[0] + mov[0], pos[1] + mov[1])
        path.append(pos)
        new = data[pos[1]][pos[0]]
        match new:
            case 'S':
                return path
            case 'L':
                if mov == (0, 1):
                    mov = (1, 0)
                elif mov == (-1, 0):
                    mov = (0, -1)
            case 'J':
                if mov == (0, 1):
                    mov = (-1, 0)
                elif mov == (1, 0):
                    mov = (0, -1)
            case 'F':
                if mov == (0, -1):
                    mov = (1, 0)
                elif mov == (-1, 0):
                    mov = (0, 1)
            case '7':
                if mov == (0, -1):
                    mov = (-1, 0)
                elif mov == (1, 0):
                    mov = (0, 1)
            case _:
                print(new)    
    
path = pathfinder(data, pos, mov)

list = []
i = 0
pathy = Path(path)
for y in range(len(data)):
    for x in range(len(data[y])):
        if (x, y) not in path and pathy.contains_point((x, y)):
            i += 1
            list.append((x, y))

data = open('data10.4.txt', 'w')
data.write(str(list))
print(i)