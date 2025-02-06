from pprint import pprint

with open('input.txt', 'r', encoding='utf-8') as f:
    reports = f.readlines()

unsafe = []

results = {}

for rep in reports:
    current_dir = None
    prev_dir = None
    clear = False
    bad = 0
    break_lvliter = False
    levels = list(map(int, rep.split()))
    for n, _ in enumerate(levels):
        if break_lvliter:
            break_lvliter = False
            break
        print(n, _)
        while not clear:
            prev_dir = current_dir
            if n + 1 == len(levels):
                break
            a = levels[n]
            b = levels[n + 1]
            current_dir = 'up' if b > a else 'down'
            if prev_dir is None:
                prev_dir = current_dir
            # Check safety
            if (prev_dir != current_dir) or (abs(a - b) not in range(1, 4)):
                bad += 1
                print(f'BAD n{bad}: ({a}, {b}) of', rep.strip(), prev_dir, current_dir, prev_dir != current_dir, abs(a - b), abs(a - b) not in range(1, 4))
                if bad > 1:
                    print('UNSAFE!')
                    unsafe.append(rep)
                    break_lvliter = True
                    break
                else:
                    levels.remove(b)
                    print(levels)
                    continue
            clear = True
    results[rep] = (bad, rep in unsafe)

safe = [rep for rep in reports if rep not in unsafe]
print(len(safe))
