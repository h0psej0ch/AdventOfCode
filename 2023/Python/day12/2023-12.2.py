import itertools

data = open('data12.txt').read().split('\n')
data2 = open('data12.3.txt', 'w')

def check2(input, num):
    input = input.split('.')
    input = [i for i in input if i]
    for i in range(len(num)):
        if int(num[i]) != len(input[i]):
            return False
    return True

def make(count):
    return [''.join(p) for p in itertools.product(['.', '#'], repeat=count) if p.count("#") == (numcount - hashcount)]

sum = 0
sumlist = []
for i in data:
    partsum = 0
    multsum = 0
    multsum2 = 0
    copylist = []
    print(data.index(i))
    input = i.split(' ')[0]
    num = i.split(' ')[1]
    numlist = num.split(',')
    numcount = int(0)
    hashcount = input.count("#")
    for l in numlist:
        numcount += int(l)
    tries = make(input.count("?"))
    failure = 0
    for k in tries:
        copy = str(input)
        for j in range(len(k)):
            copy = copy.replace('?', k[j], 1)
        if check2(copy, numlist):
            partsum += 1

    input2 = '?' + input
    if input[-1] != '#':
        tries2 = make(input2.count("?"))
    else:
        tries2 = make(input2.count("?") - 1)
        for j in range(len(tries2)):
            tries2[j] = '.' + tries2[j]
    failure = 0
    for k in tries2:
        copy = str(input2)
        for j in range(len(k)):
            copy = copy.replace('?', k[j], 1)
        if check2(copy, numlist):
            copylist.append(copy[1:])
            multsum += 1

    input3 = input + '?'
    if input[0] != '#':
        tries3 = make(input3.count("?"))
    else:
        tries3 = make(input3.count("?") - 1)
        for j in range(len(tries3)):
            tries3[j] = tries3[j] + '.'
    failure = 0
    for k in tries3:
        copy = str(input3)
        for j in range(len(k)):
            copy = copy.replace('?', k[j], 1)
        if copy[:len(copy)-1] not in copylist and check2(copy, numlist):
            multsum2 += 1
    print(str(partsum) + " partsum, " + str(multsum) + " multsum, " + str(multsum2) + " multsum2")
    sumlist.append(partsum * ((multsum + multsum2) ** 4))
    sum += partsum * ((multsum + multsum2) ** 4)
data2.write(str(sumlist))
print(sum)