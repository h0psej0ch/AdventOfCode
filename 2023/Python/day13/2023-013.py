data = open("data13.txt").read().split("\n\n")

def gethor(puzzle):
    def puzzling(puzzle):
        for i in range(len(puzzle)-1):
            found = True
            j = 0
            while i-j >= 0 and j+i+1 < len(puzzle):
                fst = puzzle[i-j]
                snd = puzzle[i+j+1]
                if fst == snd:
                    j += 1
                else:
                    found = False
                    break
            if found:
                return i
        return -1
    
    numlist = (puzzling(puzzle), "default")

    for i in range(len(puzzle)):
        for j in range(len(puzzle[i])):
            copy = puzzle.copy()
            if copy[i][j] == ".":
                copy[i] = copy[i][:j] + "#" + copy[i][j+1:]
            else:
                copy[i] = copy[i][:j] + "." + copy[i][j+1:]
            if copy[i] in puzzle:
                # print(copy[i])
                # print("\n")
                # for i in puzzle:
                #     print(i)
                # print("\n")
                copied = puzzling(copy)
                if copied != -1:
                    return (puzzling(copy), "dif")
    return numlist

sum = 0
for puzzle in data:
    puzzlelist = puzzle.split("\n")
    puzzlelist = [i for i in puzzlelist if i]
    verlist = list(zip(*puzzlelist[::-1]))
    for i in range(len(verlist)):
        copy = ""
        for j in range(len(verlist[i])):
            copy += verlist[i][j]
        verlist[i] = copy
    hor = gethor(puzzlelist)
    ver = gethor(verlist)
    if hor[1] == "dif" and ver[1] == "dif":
        if hor[0] >= ver[0]:
            sum += (hor[0]+1) * 100
        else:
            sum += (ver[0]+1)
    elif hor[1] == "dif":
        sum += (hor[0]+1) * 100
    elif ver[1] == "dif":
        sum += (ver[0]+1)
    else:
        sum += (hor[0]+1) * 100 + (ver[0]+1)
print(sum)