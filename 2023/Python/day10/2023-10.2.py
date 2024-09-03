data = open('data10.txt').read().split('\n')
data2 = open('data10.3.txt', 'w')

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
    path.append((pos, mov))
    pos = (pos[0] + mov[0], pos[1] + mov[1])
    while True:
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
        path.append((pos, mov))
        pos = (pos[0] + mov[0], pos[1] + mov[1])
    
path = pathfinder(data, pos, mov)

pathcopy = path.copy()

for i in range(len(path)):
    path[i] = (path[i][0], path[i][1], (path[i-1][1], path[i][1]))

path.sort(key=lambda x: x[0][1])

splitted = []

default = path[0][0][1]
defaulti = 0

for i in range(len(path)):
    if path[i][0][1] != default:
        splitted.append(path[defaulti:i])
        default = path[i][0][1]
        defaulti = i
splitted.append(path[defaulti:len(path)])

for k in range(len(splitted)):
    i = 0
    while i < len(splitted[k]):
        if splitted[k][i][1] == (1,0):
            j = i
            while splitted[k][j][1] == (1,0):
                j += 1
            splitted[k] = splitted[k][:i] + [((min(splitted[k][i][0][0], splitted[k][j][0][0]), max(splitted[k][i][0][0], splitted[k][j][0][0])), splitted[k][i][0][1], (splitted[k][i][2][0], splitted[k][j][2][1]))] + splitted[k][j+1:]
        elif splitted[k][i][1] == (-1,0):
            j = i
            while splitted[k][j][1] == (-1,0):
                j += 1
            splitted[k] = splitted[k][:i] + [((min(splitted[k][i][0][0], splitted[k][j][0][0]), max(splitted[k][i][0][0], splitted[k][j][0][0])), splitted[k][i][0][1], (splitted[k][j][2][1], splitted[k][i][2][0]))] + splitted[k][j+1:]
        else:
            splitted[k][i] = ((splitted[k][i][0][0], splitted[k][i][0][0],), splitted[k][i][0][1], splitted[k][i][2]) 
        i += 1

sum = 0

# Removing the points that go back from where they were and thus cant have something inside
for level in splitted:
    i = 0
    while i < len(level):
        if level[i][2][0][1] == -(level[i][2][1][1]):
            level.pop(i)
        else:
            level[i] = (level[i][0], level[i][1])
            i += 1

gaplist = []

for k in range(len(splitted)):
    ylevel = splitted[k]
    print(ylevel)
    lengy = len(ylevel)
    if lengy%2 == 0:
        i = 0
        while i < lengy:
            begin = ylevel[i][0][1]
            print("begin " + str(begin))
            end = ylevel[i+1][0][0]
            print("end " + str(end))
            for l in range((ylevel[i][0][1]+1), ylevel[i+1][0][0]):
                print(l)
                gaplist.append((l, ylevel[i][1]))
            sum += ylevel[i+1][0][0] - ylevel[i][0][1] - 1
            i += 2 
    else:
        print("how???")
        print(ylevel)
        i = 1
        sum += ylevel[0][0]
        while i < len(ylevel):
            sum += ylevel[i+1][0] - ylevel[i][0] - 1
            i += 2

print(sum)

i = 0
while i < len(gaplist):
    found = False
    for point in pathcopy:
        if point[0] == gaplist[i]:
            gaplist.pop(i)
            found = True
            break
    if found == False:
        i += 1

print(gaplist)


print("buuuuuuuut" + str(len(gaplist)))