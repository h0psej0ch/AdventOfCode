dat = open("data4.txt", 'r')

tot = 0
for x in dat:
    c = x.find(",")
    fst = x[:c]
    snd = x[c+1:]
    dashf = fst.find("-")
    dashs = snd.find("-")
    ffst = int(fst[:dashf])
    lfst = int(fst[dashf+1:])
    fsnd = int(snd[:dashs])
    lsnd = int(snd[dashs+1:])

    print("NEW: " + str(x) + "\nWhere fst: " + str(ffst) + " and " + str(lfst) + "\nWhere snd: " + str(fsnd) + " and " + str(lsnd))
    if (ffst >= fsnd and ffst <= lsnd):
        tot += 1
    elif (lfst >= fsnd and lfst <= lsnd):
        print("YASS")
        tot += 1
    else:
        print("NOT: " + str(x))
    
print(tot)