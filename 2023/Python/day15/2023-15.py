data = open('data15.txt').read().split(',')

def hashnum(num):
    current = 0
    for char in num:
        current += ord(char)
        current *= 17
        current %= 256
    return current

print(sum(hashnum(block) for block in data))
