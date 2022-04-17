from collections import Counter


def check(x):
    c = Counter(str(x))
    return all(n <= 3 for n in c.values())


N = 6
result = sum(map(check, range(10**(N-1), 10**N)))
print(result)
