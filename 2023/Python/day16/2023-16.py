data = open('data16.txt').read().split('\n')

dir = (1, 0)
pos = (0, 0)

def calcenergy(pos, dir):
    def path(dir, pos):
        while pos[1] >= 0 and pos[1] < len(data) and pos[0] >= 0 and pos[0] < len(data[pos[1]]):
            energizedset.add((dir, pos))
            current = data[pos[1]][pos[0]]
            match current:
                case "|":
                    if abs(dir[0]) == 1:
                        if ((0, 1), (pos[0], pos[1] + 1)) not in todolist:
                            todolist.append(((0, 1), (pos[0], pos[1] + 1)))
                        dir = (0, -1)
                case "-":
                    if abs(dir[1]) == 1:
                        if ((1, 0), (pos[0] + 1, pos[1])):
                            todolist.append(((1, 0), (pos[0] + 1, pos[1])))
                        dir = (-1, 0)
                case "\\":
                    dir = (dir[1], dir[0])
                case "/":
                    dir = (-dir[1], -dir[0])
            pos = (pos[0] + dir[0], pos[1] + dir[1])
            if (dir, pos) in energizedset:
                return
    
    energizedset = set()
    todolist = [(dir, pos)]

    for i in todolist:
        path(i[0], i[1])

    final = set()

    for i in energizedset:
        final.add(i[1])
    return len(final)

def highest():
    highest = calcenergy(pos, dir)
    for i in range(len(data)):
        left = calcenergy((0, i), (1, 0))
        right = calcenergy((len(data[i])-1, i), (-1, 0))
        if left > highest:
            highest = left
            print("left: " + str(i))
        if right > highest:
            highest = right
            print("right: " + str(i))

    for i in range(len(data[0])):
        above = calcenergy((i, 0), (0, 1))
        below = calcenergy((i, len(data)-1), (0, -1))
        if above > highest:
            highest = above
            print("above: " + str(i))
        if below > highest:
            highest = below
            print("under: " + str(i))
    return highest

print("Puzzle 1: " + str(calcenergy(pos, dir)))
print("Puzzle 2: " + str(highest()))
