data = open("data5.txt", 'r')
list = []
for x in data:
    list.append(x)
print(list)
locold = -1
seeds = (list[0][7:].split())
soilp = list.index("seed-to-soil map:\n")
fertp = list.index("soil-to-fertilizer map:\n")
waterp = list.index("fertilizer-to-water map:\n")
lightp = list.index("water-to-light map:\n")
tempp = list.index("light-to-temperature map:\n")
hump = list.index("temperature-to-humidity map:\n")
locp = list.index("humidity-to-location map:\n")
print(soilp)
print(fertp)
print(waterp)
print(lightp)
print(tempp)
print(hump)
print(locp)
j = 0
print(str(len(seeds)) + "wut" )
while j < (len(seeds)):
    seed = int(seeds[j])
    while seed < int(seeds[j]) + int(seeds[j+1]):
        soil = int(seed)
        for i in range(soilp+1, fertp-1):
            if int(seed) >= int(list[i].split()[1]) and int(seed) <= int(list[i].split()[1]) + int(list[i].split()[2]):
                soil = int(list[i].split()[0]) + int(seed) - int(list[i].split()[1])
                break
        fert = soil
        for i in range(fertp+1, waterp-1):
            if int(soil) >= int(list[i].split()[1]) and int(soil) <= int(list[i].split()[1]) + int(list[i].split()[2]):
                fert = int(list[i].split()[0]) + int(soil) - int(list[i].split()[1])
                break
        water = fert
        for i in range(waterp+1, lightp-1):
            if fert >= int(list[i].split()[1]) and fert <= int(list[i].split()[1]) + int(list[i].split()[2]):
                water = int(list[i].split()[0]) + fert - int(list[i].split()[1])
                break
        light = water
        for i in range(lightp+1, tempp-1):
            if water >= int(list[i].split()[1]) and water <= int(list[i].split()[1]) + int(list[i].split()[2]):
                light = int(list[i].split()[0]) + water - int(list[i].split()[1])
                break
        temp = light
        for i in range(tempp+1, hump-1):
            if light >= int(list[i].split()[1]) and light <= int(list[i].split()[1]) + int(list[i].split()[2]):
                temp = int(list[i].split()[0]) + light - int(list[i].split()[1])
                break
        hum = temp
        for i in range(hump+1, locp-1):
            if temp >= int(list[i].split()[1]) and temp <= int(list[i].split()[1]) + int(list[i].split()[2]):
                hum = int(list[i].split()[0]) + temp - int(list[i].split()[1])
                break
        loc = hum
        for i in range(locp+1, len(list)):
            if hum >= int(list[i].split()[1]) and hum <= int(list[i].split()[1]) + int(list[i].split()[2]):
                loc = int(list[i].split()[0]) + hum - int(list[i].split()[1])
                break
        if locold == -1:
            locold = loc
            print("First one " + str(locold))
        if int(loc) < int(locold):
            print(str(loc) + " location")
            locold = loc
            print(str(locold) + " lowest location")
        seed += 1
    i += 2

print(locold)