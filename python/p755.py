import numpy as np

MAX = 10 ** 13

fibs = [1]
n = 2
while n <= MAX:
    fibs.append(n)
    n += fibs[-2]
fibs = np.array(fibs)
fibsums = np.concatenate([np.array([0]), np.cumsum(fibs)])

print(len(fibs), "numbers")

def S(x, max_idx=len(fibs)):
    if x < 0:
        return 0
    if x == 0:
        return 1
    if fibsums[max_idx] <= x:
        return 1 << max_idx
    return S(x - fibs[max_idx - 1], max_idx - 1) + S(x, max_idx - 1)

print(S(100))
print(S(10**4))
print(S(10**13))
