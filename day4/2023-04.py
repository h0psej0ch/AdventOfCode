data = open("data4.txt", 'r')
sum = 0
for x in data:
    count = 0
    x = x[x.find(":")+1:]
    bar = x.find("|")
    win = x[:bar-1]
    winlist = win.split()
    num = x[bar+1:]
    numlist = num.split()
    for i in winlist:
        if i in numlist:
            count += 1
    if count > 1:
        sum += 1*(2**(count-1))
    elif count == 1:
        sum += 1
print(sum)