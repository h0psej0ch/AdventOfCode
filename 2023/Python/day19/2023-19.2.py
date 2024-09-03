ops = {"<": (lambda x,y: x<y), ">": (lambda x,y: x>y)}

data = open('data19.txt').read().split('\n\n')
rules = data[0].split('\n')
parts = data[1].split('\n')

for i in range(len(rules)):
    bracket = rules[i].find('{')
    rules[i] = (rules[i][:bracket], rules[i][bracket+1:-1])

def getindex(code):
    for y in range(len(rules)):
        if str(rules[y][0]) == str(code):
            return y

start = getindex('in')

acceptedranges = []

starting = [(start, 0), [1, 4000], [1, 4000], [1, 4000], [1, 4000]]
checklist = [starting]
checkcount = 0
while checkcount < len(checklist):
    current = checklist[checkcount]
    checking = True
    print(current)
    while checking:
        print(current[0][0])
        currentrule = rules[current[0][0]]
        ruleset = currentrule[1].split(',')
        rule = ruleset[current[0][1]]
        if rule != ruleset[-1]:
            rule = rule.split(':')
            match rule[0][0]:
                case 'x': pos = 1
                case 'm': pos = 2
                case 'a': pos = 3
                case 's': pos = 4
            newcurrent = current.copy()
            newcurrent[0] = (current[0][0], int(current[0][1])+1)
            if rule[0][1] == '<':
                current[pos] = [current[pos][0], min(int(rule[0][2:])-1, current[pos][1])]
                newcurrent[pos] = [max(int(rule[0][2:]), current[pos][0]), current[pos][1]]
            else:
                current[pos] = [max(int(rule[0][2:])+1, current[pos][0]), current[pos][1]]
                newcurrent[pos] = [current[pos][0], min(int(rule[0][2:]), current[pos][1])]
            checklist.append(newcurrent)
            if rule[1] == 'A':
                checking = False
                acceptedranges.append(current[1:])
            elif rule[1] == 'R':
                checking = False
            else:
                current[0] = (getindex(rule[1]), 0)
        else:
            if rule == 'A':
                checking = False
                acceptedranges.append(current[1:])
            elif rule == 'R':
                checking = False
            else:
                current[0] = (getindex(rule), 0)
    checkcount += 1

print(acceptedranges)

i = 0
while i < len(acceptedranges):
    popped = False
    for j in range(len(acceptedranges[i])):
        print(acceptedranges[i][j][0], acceptedranges[i][j][1])
        if acceptedranges[i][j][0] > acceptedranges[i][j][1]:
            acceptedranges.pop(i)
            popped = True
            break
    if not popped:
        i += 1
print(acceptedranges)