data = open("data4.txt", 'r')
sum = 0
list = []
for y in data:
    list.append(y)
cards = []
for k in range(len(list)):
    cards.append(k)
j = 0
while int(j) < int(len(cards)):
    x = list[cards[j]]
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
    for i in range(count):
        cards.append(cards[j] + 1 + i)
        #print(cards[j] + 1 + i)
    j += 1
print(len(cards))