import numpy as np

n = np.arange(1, 100000)
t = n * (n+1) // 2
p = n * (3*n - 1) // 2
h = n * (2*n - 1)

s = set(t) & set(p) & set(h)
print(sorted(s))
