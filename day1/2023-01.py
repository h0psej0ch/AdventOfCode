import re
data = open("data1.txt", 'r')
sum = 0
for x in data:
    num = []
    for i in range (0, len(x)):
        if x[i].isdigit():
            num.append(x[i])
        else:
            match x[i]:
                case 'o':
                    if x[i+1] == 'n' and x[i+2] == 'e':
                        num.append('1')
                        i += 2
                case 't':
                    if x[i+1] == 'w' and x[i+2] == 'o':
                        num.append('2')
                        i += 2
                    elif x[i+1] == 'h' and x[i+2] == 'r' and x[i+3] == 'e' and x[i+4] == 'e':
                        num.append('3')
                        i += 4
                case 'f':
                    if x[i+1] == 'o' and x[i+2] == 'u' and x[i+3] == 'r':
                        num.append('4')
                        i += 3
                    elif x[i+1] == 'i' and x[i+2] == 'v' and x[i+3] == 'e':
                        num.append('5')
                        i += 3
                case 's':
                    if x[i+1] == 'i' and x[i+2] == 'x':
                        num.append('6')
                        i += 2
                    elif x[i+1] == 'e' and x[i+2] == 'v' and x[i+3] == 'e' and x[i+4] == 'n':
                        num.append('7')
                        i += 4
                case 'e':
                    if x[i+1] == 'i' and x[i+2] == 'g' and x[i+3] == 'h' and x[i+4] == 't':
                        num.append('8')
                        i += 4
                case 'n':
                    if x[i+1] == 'i' and x[i+2] == 'n' and x[i+3] == 'e':
                        num.append('9')
                        i += 3

                        
    
    sum += (int(num[0]) * 10 + int(num[-1]))
    print(sum)

