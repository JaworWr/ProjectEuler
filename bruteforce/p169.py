from functools import cache


N = 10**8
pows = []
p = 1
while p < N:
    pows.append(p)
    p *= 2


@cache
def ways(k=len(pows), n=N):
    if n == 0:
        return 1
    if n < 0 or k == 0:
        return 0
    res = 0
    for i in range(3):
        n1 = n - i * pows[k - 1]
        res += ways(k - 1, n1)
    return res

print(ways())
