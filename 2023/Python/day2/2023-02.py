data = open("data2.txt", 'r')
sum = 0

def pearlcheck(x, col):
    pos = True
    for i in range(0, len(x)):
        red = x.find(col)
        if red != -1:
            j = 2
            dig = x[red-j].isdigit()
            mul = 1
            pearl = 0
            while dig:
                pearl += int(x[red-j])*mul
                if (pearl > 12 and col == "red") or (pearl > 13 and col == "green") or (pearl > 14 and col == "blue"):
                    pos = False
                    break
                mul *= 10
                j += 1
                dig = x[red-j].isdigit()
            x = x[red + len(col):]
            i = red + len(col) - 1
            i += 1
        else:
            break
    return pos


count = 0
for x in data:
    pos = x.find(":")
    x = x[pos + 2:]
    found = x.find("red")
    count += 1
    if pearlcheck(x, "red") and pearlcheck(x, "green") and pearlcheck(x, "blue"):
        sum += count
    print(sum)
    

