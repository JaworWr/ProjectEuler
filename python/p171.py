from functools import cache

MOD = 10**9

@cache
def f_aux(n, coins, first=False):
    if n > coins * 81:
        return {}

    res = {}
    if n == 0:
        res[0] = (1, 0)
    if coins <= 0:
        return res

    for c in range(10):
        c2 = c ** 2
        if c == 0 and first:
            continue
        if c2 > n:
            break
        d = f_aux(n - c2, coins - 1)
        for k, (t, s) in d.items():
            t0, s0 = res.get(k + 1, (0, 0))
            t0 += t
            s0 += s + t * c * 10**k
            res[k + 1] = (t0, s0 % MOD)
    return res

def f(n, coins):
    d = f_aux(n, coins, True)
    return sum(s for _, s in d.values()) % MOD

s = sum(f(k**2, 20) for k in range(1, 41)) % MOD
print(s)
