from collections import Counter
from tqdm import tqdm

def digits_equal(x, y):
    xd = Counter(str(x))
    yd = Counter(str(y))
    return xd == yd


for x in range(1, 999999999):
    ok = True
    for m in [2, 3, 4, 5, 6]:
        if not digits_equal(x, m * x):
            ok = False
            break
    if ok:
        print(x)
        break
