cur = 0
tot = 0

list2 = open("data2.txt",  'r')
list = []

for x in list2:
    start = x[0]
    res = x[2]
    if start == "A":
        if res == "Y":
            list.append("A X")
        elif res == "Z":
            list.append("A Y")
        elif res == "X":
            list.append("A Z")
        
    elif start == "B":
        if res == "X":
            list.append("B X")
        elif res == "Y":
            list.append("B Y")
        elif res == "Z":
            list.append("B Z")
    
    elif start == "C":
        if res == "Z":
            list.append("C X")
        elif res == "X":
            list.append("C Y")
        elif res == "Y":
            list.append("C Z")


for i in list:
    cur = 0
    X = i.find("X")
    Y = i.find("Y")
    Z = i.find("Z")
    if X != -1:
        cur = 1
        if i[0] == "A":
            cur = cur + 3
        elif i[0] == "C":
            cur = cur + 6

    if Y != -1:
        cur = 2
        if i[0] == "A":
            cur = cur + 6
        elif i[0] == "B":
            cur = cur + 3

    if Z != -1:
        cur = 3
        if i[0] == "B":
            cur = cur + 6
        elif i[0] == "C":
            cur = cur + 3
    tot = tot + cur
print(tot)