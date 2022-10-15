from collections import Counter

from tqdm import tqdm


solutions = Counter()

for p in tqdm(range(3, 1001)):
    for a in range(1, p // 3 + 1):
        for b in range(a, p):
            c = p - a - b
            if c < b:
                break
            if a**2 + b**2 == c**2:
                solutions[p] += 1

print(solutions[120])
print(solutions.most_common(1))
