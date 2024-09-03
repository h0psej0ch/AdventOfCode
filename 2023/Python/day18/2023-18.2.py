data = open("data18.txt").read().split("\n")
data2 = open("data18-2.txt", "w")

beniging = [0, 0]
move = (0, 0)
pointset = []
distance = 0

for instruction in data:
    direction, amount, color = instruction.split(" ")
    match str(color)[7]:
        case "0": direction = "R"
        case "1": direction = "D"
        case "2": direction = "L"
        case "3": direction = "U"
    amount = int(str(color)[2:-2], 16)
    distance += amount
    match direction:
        case "U": move = (0,-amount)
        case "D": move = (0, amount)
        case "L": move = (-amount,0)
        case "R": move = (amount, 0)
    beniging = [beniging[0] + move[0], beniging[1] + move[1]]
    pointset.append(beniging)

summy = 0

for i in range(len(pointset)):
    summy += pointset[-i][0] * pointset[-i-1][1] - pointset[-i][1] * pointset[-i-1][0]

summy = abs(summy) // 2

print(summy + distance//2 + 1)