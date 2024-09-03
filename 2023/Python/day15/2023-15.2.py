data = open('data15.txt').read().split(',')

def hashnum(num):
    current = 0
    for char in num:
        current += ord(char)
        current *= 17
        current %= 256
    return current

boxlist = []

for block in data:
    found = block.find("=")
    if found != -1:
        operator = "="
        nonhash = block[:found]
        print(nonhash + " dit is de nonhash")
        hash = hashnum(nonhash)
        bigfind = False
        if len(boxlist) == 0:
            boxlist.append([hash, (nonhash, int(block[found+1:]))])
        else:    
            for i in boxlist:
                if i[0] == hash:
                    bigfind = True
                    finding = False
                    for j in range(1, len(i)):
                        if i[j][0] == nonhash:
                            i[j] = (nonhash, int(block[found+1:]))
                            finding = True
                            break
                    if not finding:
                        i.append((nonhash, int(block[found+1:])))
            if not bigfind:
                boxlist.append([hash, (nonhash, int(block[found+1:]))])
    else:
        found = block.find("-")
        operator = "-"
        nonhash = block[:block.find(operator)]
        hash = hashnum(nonhash)
        for box in boxlist:
            if box[0] == hash:
                for i in range(1, len(box)):
                    if box[i][0] == nonhash:
                        box.pop(i)
                        break
    print(boxlist)

summy = 0
for box in boxlist:
    for i in range(1, len(box)):
        summy += (box[0] + 1)*i*box[i][1]

print(sum(hashnum(block) for block in data))
print(summy)
