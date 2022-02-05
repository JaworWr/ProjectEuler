MAX = 50
dp = [1, 1, 1]
for i in range(3, MAX + 1):
    r = sum(dp[j] for j in range(i - 3)) + 1
    r += dp[-1]
    dp.append(r)

print(dp[-1])
