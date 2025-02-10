with open('input.txt', 'r', encoding='utf-8') as f:
    reports = f.readlines()

unsafe = []

for rep in reports:
    levels = list(map(int, rep.split()))
    first_dir = None
    for n, _ in enumerate(levels):
        if n + 1 == len(levels):
            break
        a = levels[n]
        b = levels[n + 1]
        current_dir = 'up' if b > a else 'down'
        if first_dir is None:
            first_dir = current_dir
        # Check safety
        if (first_dir != current_dir) or (abs(a - b) not in range(1, 4)):
            unsafe.append(rep)
            break

safe = [rep for rep in reports if rep not in unsafe]
print(len(safe))
