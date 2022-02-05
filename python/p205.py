from collections import defaultdict

def prob(k, n):
    dist = {0: 1}
    for i in range(n):
        d1 = defaultdict(lambda: 0)
        for x in range(1, k + 1):
            for y, c in dist.items():
                d1[y + x] += c
        dist = dict(d1)
    return dist

d1 = prob(4, 9)
d2 = prob(6, 6)
p = 0
for k1, v1 in d1.items():
    for k2, v2 in d2.items():
        if k1 > k2:
            p += v1 * v2
print(f"{p / (sum(d1.values()) * sum(d2.values())):.7}")
