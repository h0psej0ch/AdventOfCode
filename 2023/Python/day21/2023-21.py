data = open('data21.txt').read().split('\n')
data2 = open('data21.2.txt', 'w')

for line in range(len(data)):
    found = data[line].find('S')
    if found != -1:
        start = [line, found]
        break
print(start)

points = [start]
count = 0
pointsoldold =[]
pointsold = []
even = []
odd = []
while count < 64:
    pointsoldold = pointsold
    pointsold = points
    copylist =[]
    for point in points:
        try:
            if data[point[0]][point[1]+1] == '.' or data[point[0]][point[1]+1] == 'S':
                copylist.append([point[0], point[1]+1])
        except:
            pass
        try:
            if data[point[0]][point[1]-1] == '.' or data[point[0]][point[1]-1] == 'S':
                copylist.append([point[0], point[1]-1])
        except:
            pass
        try:
            if data[point[0]+1][point[1]] == '.' or data[point[0]+1][point[1]] == 'S':
                copylist.append([point[0]+1, point[1]])
        except:
            pass
        try:
            if data[point[0]-1][point[1]] == '.' or data[point[0]-1][point[1]] == 'S':
                copylist.append([point[0]-1, point[1]])
        except:
            pass
    points = []
    for point in copylist:
        if point not in points:
            if point in pointsoldold and count%2 ==0:
                odd.append(point)
            elif point in pointsoldold and count%2 ==1:
                even.append(point)
            else:
                points.append(point)
    print(count)
    count += 1

printline = ''
if count %2 == 0:
    dataset = even
else:
    dataset = odd
for line in range(len(data)):
    for char in range(len(data[line])):
        if [line, char] in points or [line, char] in dataset:
            printline += 'O'
        else:
            printline += data[line][char]
    printline += '\n'

data2.write(printline)

print(points)

print(len(points))