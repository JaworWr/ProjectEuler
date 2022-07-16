from tqdm import tqdm


def pdp(x, n):
    res = ""
    for y in range(1, n+1):
        res += str(x * y)
    return res

m = ""
for x in range(99999):
    for n in range(999):
        p = pdp(x, n)
        if len(p) < 9:
            continue
        if len(p) > 9:
            break
        if set(p) != set("123456789"):
            continue
        m = max(m, p)
print(m)
