cal = 0
calold1 = 0
calold2 = 0
calold3 = 0

list = open("data.txt",  'r')
for x in list:
    cur = x[:-1]
    if cur != "":
        cal = cal + int(cur)
    else: 
        if cal > calold1:
            calold3 = calold2
            calold2 = calold1
            calold1 = cal
        elif cal > calold2:
            calold3 = calold2
            calold2 = cal
        elif cal > calold3:
            calold3 = cal
        print(calold1)
        print(calold2)
        print(str(calold3) + "\n")
        cal = 0


print(int(calold1 + calold2 + calold3))