import math

data = open('data8.txt').read().split('\n')
instr = data[0]
mapy = data[2:]

temp = []
for block in mapy:
    for i in range(len(mapy)):
        if mapy[i].find(block[7:10]) == 0:
            left = i
        if mapy[i].find(block[12:15]) == 0:
            right = i
    temp.append((block[0:3], left, right))

print(temp)

cur = []
beniging = 0
for i in range(len(temp)):
    if temp[i][0][2] == 'A':
        cur.append((temp[i][0:3], i))

def loop(start):
    i = -1
    zlist = []
    while True:

        i += 1
        ins = instr[i % len(instr)]
        if ins == 'L':
            start = (temp[temp[start[1]][1]], temp[start[1]][1])
        elif ins == 'R':
            start = (temp[temp[start[1]][2]], temp[start[1]][2])
        if start[0][0][2] == 'Z':
            print(start[0][0])
            for j in range(len(zlist)):
                if start[0][0] in zlist[j]:
                    found = j
                    print(i)
                    return ((i-zlist[found][1]), zlist[found][1])
            
            zlist.append((start[0][0], i))
            print(zlist)

listy = []
for start in cur:
    listy.append(loop(start)[0])

print(listy)

lcm = (listy[0]*listy[1])//math.gcd(listy[0], listy[1])
for b in listy[2:]:
    lcm = (lcm*b)//math.gcd(lcm, b)

print("LCM:", lcm)
