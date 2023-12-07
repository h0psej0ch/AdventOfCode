data = open("data3.txt", 'r')
list=[]
sum = 0
for x in data:
    list.append(x)

def getdig(x, l):
    dig = 0
    mul = 1
    j = 1
    digit = True
    while digit:
        dig += int(x[l-j]) * mul
        j += 1
        mul *= 10
        digit = x[l-j].isdigit()
    return dig

def firstquest(sum):
    for i in range(0, len(list)):
        x = list[i]
        l = 0
        while l < len(x):
            if x[l].isdigit():
                dig = int(x[l])
                continue_flag = True
                j = 1
                while continue_flag:
                    if x[l+j].isdigit():
                        dig *= 10
                        dig += int(x[l+j])
                        j += 1
                    else:
                        add = False
                        for k in range(l-1, l+j+1):
                            #print(list[i+1][k] + list[i][k] + list[i-1][k] + " This is column " + str(k-l))
                            if k >= 0 and k < len(x)-1:
                                if i > 0 and list[i-1][k] != ".":
                                    print(list[i-1][k])
                                    add = True
                                if i < len(list)-1 and list[i+1][k] != ".":
                                    print(list[i+1][k])
                                    add = True
                        if l-1 >= 0  and x[l-1] != ".":
                            print(x[l-1])
                            add = True
                        if (l+j < len(x)-1) and (x[l+j] != "."):
                            if dig == 166:
                                print(l+j)
                                print(len(x))
                            print(x[l+j])
                            add = True
                        if add == True:
                            sum += dig
                            print(dig)
                        continue_flag = False
                l += j
            else:
                l += 1
    return sum

def secondquest(sum):
    for i in range(0, len(list)):
        x = list[i]
        l = 0
        while l < len(x):
            if x[l] == "*":
                dig = []
                for k in range (i-1, i+2):
                    if k >= 0 and k < len(list):
                        if list[k][l-1].isdigit() and not list[k][l].isdigit():
                            dig.append(getdig(list[k], l))
                        if list[k][l].isdigit() and not list[k][l+1].isdigit():
                            dig.append(getdig(list[k], l+1))
                        if list[k][l+1].isdigit():
                            found = True
                            j = 1
                            while found:
                                found = list[k][l+1+j].isdigit()
                                j+=1
                            dig.append(getdig(list[k], l+j))
                if len(dig) == 2:
                    sum += dig[0] * dig[1]
            l+=1
    return sum




print(secondquest(sum))
        
