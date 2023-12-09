data = open("data9.txt").read().split("\n")
sum = 0

def zero(x):
    for i in x:
        if i != 0:
            return False
    return True

for line in data:
    line = line.split()
    line = [int(i) for i in line]
    biglist = []
    biglist.append(line)
    while not zero(line):
        temp = []
        for j in range(len(line)-1):
            temp.append(int(line[j+1])-int(line[j]))
        biglist.append(temp)
        line = temp
    for val in range(len(biglist)-1):
        listval = -(val+1)
        new = biglist[listval-1][0] - biglist[listval][0]
        biglist[listval-1].insert(0, new)

    print(biglist[0])
    sum += biglist[0][0]
print(sum)