import matplotlib.pyplot as plt
from matplotlib.path import Path
import matplotlib.patches as patches

data = open("data18.txt").read().split("\n")
data2 = open("data18-2.txt", "w")

beniging = [0, 0]
move = (0, 0)
pointset = []
pointset.append(beniging)
for instruction in data:
    
    direction, amount, color = instruction.split(" ")
    match direction:
        case "U": move = (0,-1)
        case "D": move = (0, 1)
        case "L": move = (-1,0)
        case "R": move = (1, 0)
    for i in range(int(amount)):
        beniging = [beniging[0] + move[0], beniging[1] + move[1]]
        pointset.append(beniging)

sum = 0
path = Path(pointset)
for y in range(-120, 300):
    for x in range(-10, 400):
        if [x, y] not in pointset and path.contains_point((x, y)):
            sum += 1
sum += len(pointset) -1

thisrow = ""
for i in range(-120, 300):
    for j in range(-10, 400):
        if [j, i] in pointset:
            thisrow += "#"
        else:
            thisrow += "."
    thisrow += "\n"
data2.write(thisrow)

print(pointset)

print(sum)

fig, ax = plt.subplots()
patch = patches.PathPatch(path, facecolor='orange', lw=10)
ax.add_patch(patch)
ax.set_xlim(-10, 400)
ax.set_ylim(-120, 300)
plt.show()