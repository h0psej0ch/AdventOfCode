import itertools

data = open('data12.txt').read().split('\n')

def check2(input, num):
    input = input.split('.')
    input = [i for i in input if i]
    for i in range(len(num)):
        if int(num[i]) != len(input[i]):
            return False
    return True

def check(input, num):
    j = 0
    while j < len(num):
        if input == "":
            return False
        number = num[j]
        match = ""
        for k in range(int(number)):
            match += "#"
        found = input.find(match)
        if found == -1:
            return False
        if found == 0 and input[len(match)] == ".":
            input = input[found + len(match):]
        elif found == (len(input) - len(match)) and input[found - 1] == ".":
            input = input[found + len(match):]
        elif found != -1 and input[found - 1] == "." and input[found + len(match)] == ".":
            input = input[found + len(match):]
        else:
            return False
        j += 1
    return True

def make(count):
    return [''.join(p) for p in itertools.product(['.', '#'], repeat=count)]

sum = 0
for i in data:
    print(data.index(i))
    input = i.split(' ')[0]
    num = i.split(' ')[1]
    numlist = num.split(',')
    numcount = int(0)
    for l in numlist:
        numcount += int(l)
    tries = make(input.count("?"))
    failure = 0
    hashcount = input.count("#")
    while failure < len(tries):
        if tries[failure].count("#") + hashcount != numcount:
            tries.pop(failure)
        else:
            failure += 1
    for k in tries:
        copy = str(input)
        for j in range(len(k)):
            copy = copy.replace('?', k[j], 1)
        if check2(copy, numlist):
            sum += 1
print(sum)