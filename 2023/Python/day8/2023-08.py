data = open('data8.txt').read().split('\n')
instr = data[0]
mapy = data[2:]

beniging = 0
for i in range(len(mapy)):
    if mapy[i].find('AAA') == 0:
        cur = 'AAA'
        index = i

steps = 0

while cur != 'ZZZ':
    for ins in instr:
        steps += 1
        if ins == 'L':
            cur = mapy[index][7:10]
            for i in range(len(mapy)):
                if mapy[i].find(cur) == 0:
                    index = i
                    break
        elif ins == 'R':
            cur = mapy[index][12:15]
            for i in range(len(mapy)):
                if mapy[i].find(cur) == 0:
                    index = i
                    break
print(steps)