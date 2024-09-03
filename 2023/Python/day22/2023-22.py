data = open('data22.txt').read().split('\n')

for i in range(len(data)):
    data[i] = data[i].split('~')
    for j in range(len(data[i])):
        data[i][j] = data[i][j].split(',')
        data[i][j] = [int(data[i][j][0]), int(data[i][j][1]), int(data[i][j][2])]

for i in range(len(data)):
    if data[i][0][2] > data[i][1][2]:
        data[i][0], data[i][1] = data[i][1], data[i][0]
data.sort(key=lambda x: x[0][2])

for block in data:
    footprint = []
    for i in range(block[0][0], block[1][0] + 1):
        for j in range(block[0][1], block[1][1] + 1):
            footprint.append((i, j))
    print(block, footprint)