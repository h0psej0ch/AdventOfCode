ops = {"<": (lambda x,y: x<y), ">": (lambda x,y: x>y)}

data = open('data19.txt').read().split('\n\n')
rules = data[0].split('\n')
parts = data[1].split('\n')

for i in range(len(rules)):
    bracket = rules[i].find('{')
    rules[i] = (rules[i][:bracket], rules[i][bracket+1:-1])

def accepting(current, part):
    accepted = False
    print("NEW PART " + str(part))
    partrules = part[1:-1].split(',')
    for i in range(len(partrules)):
        partrules[i] = int(partrules[i][2:])
    searching = True
    while searching:
        currentrule = rules[current]
        ruleset = currentrule[1].split(',')
        for rule in ruleset:
            newrule = False
            if rule != ruleset[-1]:
                rule = rule.split(':')
                match rule[0][0]:
                    case 'x': pos = 0
                    case 'm': pos = 1
                    case 'a': pos = 2
                    case 's': pos = 3
                if ops[rule[0][1]] (partrules[pos], int(rule[0][2:])):
                    next = rule[1]
                    if next == 'A':
                        accepted = True
                        searching = False
                        newrule = True
                        break
                    elif next == 'R':
                        searching = False
                        newrule = True
                        break
                    else:
                        for y in range(len(rules)):
                            if str(rules[y][0]) == str(next):
                                current = y
                                newrule = True
                                break
            else:
                if rule == 'A':
                    accepted = True
                    searching = False
                elif rule == 'R':
                    searching = False
                else:
                    for y in range(len(rules)):
                        if str(rules[y][0]) == str(rule):
                            current = y
                            newrule = True
                            break
            if newrule:
                break
    return accepted

start = [y[0] for y in rules].index('in')

summation = 0
for part in parts:
    print(part)
    if accepting(start, part):
        part = part[1:-1].split(',')
        for i in part:
            summation += int(i[2:])

print(summation)