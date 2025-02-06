with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.readlines()

left = []
right = []

for pair in [line.split() for line in content]:
    left.append(pair[0])
    right.append(pair[1])

left.sort()
right.sort()

total = 0

for l, r in zip(left, right):
    total += abs(int(l) - int(r))

print(total)
