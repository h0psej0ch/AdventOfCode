data = open("data7.txt").read().split('\n')

i = 0
listy = []
for dat in data:
    dat = dat.split(' ')
    val = list(str(dat[0]))
    val = list(map(lambda x: x.replace('T', '10'), val))
    val = list(map(lambda x: x.replace('J', '11'), val))
    val = list(map(lambda x: x.replace('Q', '12'), val))
    val = list(map(lambda x: x.replace('K', '13'), val))
    val = list(map(lambda x: x.replace('A', '14'), val))
    val = [int(i) for i in val]
    listy.append((val, dat[1], i))
    i += 1
listy.sort()

five = []
four = []
full = []
three = []
twop = []
onep =[]
high = []
for i in listy:
    use = i[0]
    use.sort()
    count = 0
    old = 0
    oldold = 0
    for j in range(len(use)-1):
        if use[j] == use[j+1]:
            count += 1
        else:
            if count > old:
                oldold = old
                old = count
            elif count > oldold:
                oldold = count
            count = 0
    if count > old:
        oldold = old
        old = count
    elif count > oldold:
        oldold = count
    match old:
        case 4:
            five.append(i)
        case 3:
            four.append(i)
        case 2:
            if oldold == 1:
                full.append(i)
            else:
                three.append(i)
        case 1:
            if oldold == 1:
                twop.append(i)
            else:
                onep.append(i)
        case 0:
            high.append(i)
result = []
for i in high:
    result.append(i)
for i in onep:
    result.append(i)
for i in twop:
    result.append(i)
for i in three:
    result.append(i)
for i in full:
    result.append(i)
for i in four:
    result.append(i)
for i in five:
    result.append(i)

i = 1
sum = 0

print(result)

while i <= len(result):
    print(result[i-1][1])
    sum += i * int(result[i-1][1])
    i += 1
    
print(sum)
