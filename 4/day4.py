with open('input.txt', 'r', encoding='utf-8') as f:
    content = f.read()

wordmatrix: list[list[str]] = [list(line) for line in content.split()]

