data = open('data20.txt').read().split('\n')

for i in range(len(data)):
    data[i] = data[i].split(' -> ')

def findindex(key):
    for i in range(len(data)):
        if data[i][0][1:] == key:
            return i
    
broadcast = findindex('roadcaster')

statelist = []
def statereset():
    global statelist
    statelist = []
    for i in data:
        statelist.append([i[0][1:], 0])

# Structure: [[Conjuctionindex, [[first input, state], [second input, state], ...]], [Conjuctionindex, [[first input, state], [second input, state], ...]], ...]
conjunctionmemory =[]
conlist = []
for i in data:
    if i[0][0] == '&':
        conjunctionmemory.append([data.index(i), [], []])
        conlist.append(i[0][1:])
for i in data:
    for j in i[1].split(', '):
        if j in conlist:
            conjunctionmemory[conlist.index(j)][1].append([data.index(i), 0])

print(conjunctionmemory)

def find_index(list_of_lists, value):
    for i, sublist in enumerate(list_of_lists):
        if sublist[0] == value:
            return i
    return -1

def buttonpress():
    global count
    checklist = []
    for dir in data[broadcast][1].split(', '):
        checklist.append((findindex(dir), 0, broadcast))

    checkindex = 0
    while checkindex < len(checklist):
        current = checklist[checkindex]
        if current[0] is not None:
            if data[current[0]][0][0] == '%':
                if current[1] == 0:
                    if statelist[current[0]][1] == 0:
                        statelist[current[0]][1] = 1
                        for i in data[current[0]][1].split(', '):
                            checklist.append((findindex(i), 1, current[0]))
                    else:
                        statelist[current[0]][1] = 0
                        for i in data[current[0]][1].split(', '):
                            checklist.append((findindex(i), 0, current[0]))

            elif data[current[0]][0][0] == '&':
                index = find_index(conjunctionmemory, current[0])
                if index == -1:
                    conjunctionmemory.append([current[0], [[current[2], current[1]]]])
                else:
                    found = find_index(conjunctionmemory[index][1], current[2])
                    if found != -1:
                        conjunctionmemory[index][1][found][1] = current[1]
                    else:    
                        conjunctionmemory[index][1].append([current[2], current[1]])
                high = True
                for i in conjunctionmemory[index][1]:
                    if i[1] == 0:
                        high = False
                if high:
                    conjunctionmemory[index][2].append(count)
                    for i in data[current[0]][1].split(', '):
                        if current[0] == rxindex:
                            return "wtf"
                        checklist.append((findindex(i), 0, current[0]))
                else:
                    for i in data[current[0]][1].split(', '):
                        checklist.append((findindex(i), 1, current[0]))             
        checkindex += 1
    return

rxindex = findindex("dt")
print(str(rxindex) + " this is the index for dt")

global count
count = 0
while count < 4:
    statereset()
    new = buttonpress()
    if new != None:
        print(count)
        break
    print(count)
    for i in conjunctionmemory:
        print(i)
    count += 1

for con in conjunctionmemory:
    print(con[0], con[2])
