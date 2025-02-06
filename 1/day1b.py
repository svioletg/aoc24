with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.readlines()

left = []
right = []

for pair in [line.split() for line in content]:
    left.append(int(pair[0]))
    right.append(int(pair[1]))

left.sort()
right.sort()

score = 0

for l, r in zip(left, right):
    score += l * len([i for i in right if i == l])

print(score)
