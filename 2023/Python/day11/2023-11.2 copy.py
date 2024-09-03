data = open("data11.txt").read().split("\n")
data2 = open("data11.2.txt", "w")

rowlist =[]
collist = []

i = 0
while i < len(data):
    if '#' not in data[i]:
        rowlist.append(i)
    i += 1

j = 0
while j < len(data[0]):
    empty = True
    for i in range(len(data)):
        if data[i][j] == '#':
            empty = False
            break
    if empty:
        collist.append(j)
    j += 1

data2.write("\n".join(data))

points = []

for i in range(len(data)):
    for j in range(len(data[i])):
        if data[i][j] == '#':
            points.append((i, j))
            
sum = 0

print(rowlist)
print(collist)

def ypass(point, point2):
    minval = min(point[1], point2[1])
    maxval = max(point[1], point2[1])
    for mini in range(len(collist)):
        if collist[mini] > minval:
            break
    for maxi in range(mini, len(collist)):
        if collist[maxi] > maxval:
            break
    return maxi - mini

def xpass(point, point2):
    minval = min(point[0], point2[0])
    maxval = max(point[0], point2[0])
    for mini in range(len(rowlist)):
        if rowlist[mini] > minval:
            break
    for maxi in range(mini, len(rowlist)):
        if rowlist[maxi] > maxval:
            break
    return maxi - mini

for point in range(len(points)):
    for point2 in range(point + 1, len(points)):
        sum += abs(points[point2][0] - points[point][0]) + abs(points[point2][1] - points[point][1])
        passed = ypass(points[point], points[point2]) + xpass(points[point], points[point2])
        sum += passed*(1000000-1)
print(sum)