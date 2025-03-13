with open('input.txt', 'r', encoding='utf-8') as f:
    puzzle: str = f.read()

locids: list[tuple[int, int]] = [
    tuple(map(int, i.split())) for i in puzzle.split('\n') \
    if i.strip() != ''
]

left: list[int] = sorted(i[0] for i in locids)
right: list[int] = sorted(i[1] for i in locids)

dist: int = 0
simscore: int = 0

for a, b in zip(left, right):
    dist += abs(a - b)
    simscore += a * right.count(a)

print('Day 1, Part 1')
print(dist)
print('Day 1, Part 2')
print(simscore)
