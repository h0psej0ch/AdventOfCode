data = open("data7.txt").read().split('\n')

i = 0
listy = []
for dat in data:
    dat = dat.split(' ')
    val = list(str(dat[0]))
    val = list(map(lambda x: x.replace('T', '10'), val))
    val = list(map(lambda x: x.replace('J', '1'), val))
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
    jok = 0
    for j in range(len(use)-1):
        if use[j] == use[j+1]:
            count += 1
        else:
            if use[j] == 1:
                if count == 0:
                    jok = 1
                else:
                    jok = count+1
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
    old += jok
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
result.extend(high)
result.extend(onep)
result.extend(twop)
result.extend(three)
result.extend(full)
result.extend(four)
result.extend(five)

i = 1
sum = 0


while i <= len(result):
    sum += i * int(result[i-1][1])
    i += 1
    
print(sum)
