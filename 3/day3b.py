import re

import colorama
from colorama import Back, Fore, Style

colorama.init(autoreset=True)

with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.read()

MUL_PATTERN = re.compile(r"mul\((\d{1,3}),(\d{1,3})\)")
DO_DONT_PATTERN = re.compile(r"do\(\)|don\'t\(\)")

sects = {m.end() - 1: m.group() for m in DO_DONT_PATTERN.finditer(content)}
to_process = ""
enabled = True
for n, ch in enumerate(content):
    if enabled:
        to_process += ch
    if n in sects:
        enabled = sects[n] == 'do()'

total = 0
if matches := MUL_PATTERN.findall(to_process):
    total = sum(int(a) * int(b) for a, b in matches)

print(total)
