import itertools
from math import factorial
from tracemalloc import start
import numpy as np
from tqdm import tqdm


LINES = np.array([
    [0, 2, 4],
    [1, 4, 7],
    [8, 7, 6],
    [9, 6, 3],
    [5, 3, 2],
])


def check_perm(perm):
    perm = np.asarray(perm)
    s = np.sum(perm[LINES[0]])
    for l in LINES[1:]:
        if s != np.sum(perm[l]):
            return False
    return True


def permutation_string(perm):
    perm = np.asarray(perm)
    ext_vals = perm[LINES[:, 0]]
    start_pos = np.argmin(ext_vals)
    res = ""
    for i in range(5):
        for x in perm[LINES[(i + start_pos) % 5]]:
            res += str(x)
    return res


m = ""
mp = []
for perm in tqdm(itertools.permutations(range(1, 11)), total=factorial(10)):
    if not check_perm(perm):
        continue
    s = permutation_string(perm)
    if len(s) != 16:
        continue
    if s > m:
        m = s
        mp = list(perm)
print(m)
print(mp)
