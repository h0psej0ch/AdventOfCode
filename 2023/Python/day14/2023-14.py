data = open("data14.txt").read().split("\n")
data2 = open("data14.2.txt", "w")
roundlist = []
cubelist = []

for i in range(len(data)):
    for j in range(len(data[i])):
        if data[i][j] == "O":
            roundlist.append((i, j))
        elif data[i][j] == "#":
            cubelist.append((i, j))

def north():
    for i in range(len(roundlist)):
        falling = True
        while falling:
            if roundlist[i][0] > 0 and (roundlist[i][0] - 1, roundlist[i][1]) not in roundlist and (roundlist[i][0] - 1, roundlist[i][1]) not in cubelist:
                roundlist[i] = (roundlist[i][0] - 1, roundlist[i][1])
            else:
                falling = False

def south():
    for j in range(len(roundlist)):
        i = len(roundlist) - j - 1
        falling = True
        while falling:
            if roundlist[i][0] < len(data)-1 and (roundlist[i][0] + 1, roundlist[i][1]) not in roundlist and (roundlist[i][0] + 1, roundlist[i][1]) not in cubelist:
                roundlist[i] = (roundlist[i][0] + 1, roundlist[i][1])
            else:
                falling = False

def east():
    roundlist.sort(key=lambda x: x[1])
    roundlist.reverse()
    for i in range(len(roundlist)):
        falling = True
        while falling:
            if roundlist[i][1] < len(data[0])-1 and (roundlist[i][0], roundlist[i][1] + 1) not in roundlist and (roundlist[i][0], roundlist[i][1] + 1) not in cubelist:
                roundlist[i] = (roundlist[i][0], roundlist[i][1] + 1)
            else:
                falling = False
    roundlist.sort(key=lambda x: x[0])

def west():
    roundlist.sort(key=lambda x: x[1])
    for i in range(len(roundlist)):
        falling = True
        while falling:
            if roundlist[i][1] > 0 and (roundlist[i][0], roundlist[i][1] - 1) not in roundlist and (roundlist[i][0], roundlist[i][1] - 1) not in cubelist:
                roundlist[i] = (roundlist[i][0], roundlist[i][1] - 1)
            else:
                falling = False
    roundlist.sort(key=lambda x: x[0])

listylist = []
count = 0
finded = False
while count < 1000000000:
    north()
    west()
    south()
    east()
    score = sum(len(data)-i[0] for i in roundlist)
    try:
        found = listylist.index(score)
        if count - found == 1:
            listylist.append(score)
            count += 1
        elif not finded: 
            print("Found at " + str(found))
            print("The mod is " + str(count - found))
            print("The benigingin is " + str(1000000000 - count))
            skipping = ((1000000000 - count) // (count - found - 1))
            print("We are skipping an amount of " + str(skipping) + " rounds")
            count += (count-found) * skipping
            print("Count has increased to " + str(count))
            finded = True
        else:
            count += 1
    except ValueError:
        listylist.append(score)
        count += 1

temp = ""
for i in range(len(data)):
    for j in range(len(data[i])):
        if (i, j) in roundlist:
            temp += "O"
        elif (i, j) in cubelist:
            temp += "#"
        else:
            temp += "."
    temp += "\n"

print(listylist)
print(sum(len(data)-i[0] for i in roundlist))

data2.write(temp)