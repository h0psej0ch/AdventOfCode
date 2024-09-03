data = open("data6.txt", 'r')
time, dist = (data.read().split('\n'))
time = (time.split()[1:])
dist = (dist.split()[1:])
mul = 1
for i in range(len(time)):
    tim = int(time[i])
    dis = int(dist[i])
    mid = (tim//2) * (tim-tim//2)
    j = 0
    while mid > dis:
        j += 1
        mid = ((tim//2)-j) * (tim-(tim//2)+j)
    low = (tim//2)-j
    opt = tim - 2 * low - 1
    print(opt)
    mul = mul * opt
print(mul)