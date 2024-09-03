data = open("data11.txt").read().split("\n")
data2 = open("data11.2.txt", "w")

i = 0
while i < len(data):
    if '#' not in data[i]:
        data.insert(i, data[i])
        i += 1
    i += 1

j = 0
while j < len(data[0]):
    empty = True
    for i in range(len(data)):
        if data[i][j] == '#':
            empty = False
            break
    if empty:
        for i in range(len(data)):
            data[i] = data[i][:j] + '.' + data[i][j:]
        j += 1
    j += 1

data2.write("\n".join(data))

points = []

for i in range(len(data)):
    for j in range(len(data[i])):
        if data[i][j] == '#':
            points.append((i, j))
            
sum = 0

for point in range(len(points)):
    for point2 in range(point + 1, len(points)):
        sum += abs(points[point2][0] - points[point][0]) + abs(points[point2][1] - points[point][1])

print(sum)