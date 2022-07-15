import math


def newton(n, r):
    r = min(r, n - r)
    res = 1
    for x in range(n - r + 1, n + 1):
        res *= x
    res //= math.factorial(r)
    return res


c = 0
for n in range(1, 101):
    for r in range(n+1):
        if newton(n, r) > 1000000:
            c += 1
print(c)
