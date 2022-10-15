N = 3000
pentagonal = [n * (3*n - 1) // 2 for n in range(1, 3*N)]
pentagonal_s = set(pentagonal)

best = (9999999, 0, 0)
for i in range(N):
    p1 = pentagonal[i]
    for j in range(i + 1, N):
        p2 = pentagonal[j]
        s = p1 + p2
        d = p2 - p1
        if s in pentagonal_s and d in pentagonal_s:
            best = min(best, (d, p1, p2))
print(best)
