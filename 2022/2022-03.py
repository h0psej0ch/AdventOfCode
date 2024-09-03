tot = 0
count = 0
dat = open("data3.txt", 'r')
list = open("data3.1.txt", 'r')
for x in dat:
    if count == 0:
        one = x[:-1]
        count = 1
    elif count == 1:
        two = x[:-1]
        count = 2
    else:
        thr = x[:-1]
        count = 0
        list.seek(0)
        for i in list:
            num = i[2]
            onef = one.find(num)
            twof = two.find(num)
            thrf = thr.find(num)
            if onef != -1 and twof != -1 and thrf != -1:
                print(int(i[:-2]))
                print(num)
                print(ord(num))
                if num.isupper():
                    print("upper")
                    print(ord(num)-38)
                    tot += (ord(num)-38)
                else:
                    print(ord(num)-96)
                    tot += (ord(num)-96)
print(tot)



    # length = len(x)
    # fst = x[length//2:]
    # end = x[:-length//2]
    # list.seek(0)
    # for i in list:
    #     num = i[2]
    #     fstf = fst.find(num)
    #     endf = end.find(num)
    #     if fstf != -1 and endf != -1:
    #         print(int(i[:-2]))
    #         print(num)
    #         print(ord(num))
    #         if num.isupper():
    #             print("upper")
    #             print(ord(num)-38)
    #             tot += (ord(num)-38)
    #         else:
    #             print(ord(num)-96)
    #             tot += (ord(num)-96)
    #         print(tot)
        
print(tot)
