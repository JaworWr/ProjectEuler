SQMAX = 10 ** 7
MAX = SQMAX ** 2
found = set()
c = 0

e = 1
k = 1
e1 = e * (e + 1)
while e1 <= SQMAX:
    k = e
    N = e1 * k * (k + 1)
    while N <= MAX:
        found.add(N)
        k += 1
        N = e1 * k * (k + 1)
    e += 1
    e1 = e * (e + 1)

print(len(found))
