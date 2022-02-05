def fill_count(m, n):
    dp = [1] * m + [2]
    s = [i + 1 for i in range(m)]
    s.append(s[-1] + 2)
    for i in range(m + 1, n + 1):
        r = s[i - m - 1] + 1 + dp[-1]
        dp.append(r)
        s.append(s[-1] + r)
    return dp[n]

print(fill_count(3, 2))
print(fill_count(3, 3))
print(fill_count(3, 4))
print(fill_count(3, 7))
print(fill_count(3, 29))
print(fill_count(10, 56))

n = 50
while fill_count(50, n) <= 1000000:
    n += 1
print(n, fill_count(50, n), fill_count(50, n - 1))
