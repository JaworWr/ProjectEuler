import numpy as np

ys = np.array([0, 2, 0, -1, -1] * 2)
ks = np.array([1, 5, -1, 2, -2] * 2)

nuggets = set()
while len(nuggets) < 20:
    ys1 = np.zeros_like(ys)
    ks1 = np.zeros_like(ks)
    ys1[:5] = -9*ys[:5] - 4*ks[:5] - 2
    ks1[:5] = -20*ys[:5] - 9*ks[:5] - 4
    ys1[5:] = -9*ys[5:] + 4*ks[5:] - 2
    ks1[5:] = 20*ys[5:] - 9*ks[5:] + 4

    ys = ys1
    ks = ks1

    for y, k in zip(ys, ks):
        if y > 0 and k > 0:
            nuggets.add(y)

for i, y in enumerate(sorted(nuggets)):
    print(f"{i+1}: {y}")
