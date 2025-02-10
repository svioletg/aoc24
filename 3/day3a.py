import re

with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.read()

MUL_PATTERN = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")

total = 0
if matches := MUL_PATTERN.findall(content):
    total = sum(int(a) * int(b) for a, b in matches)

print(total)
