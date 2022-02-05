from decimal import Decimal
import math

l = Decimal("2." + "0" * 24)
r = Decimal("2." + "9" * 24)

def trim(s):
    s = str(s)
    r1, d, r2 = s.partition(".")
    return r1 + d + r2[:24]

def generate(theta):
    b = theta
    a = math.floor(b)
    res = str(a) + "."
    for _ in range(24):
        b = math.floor(b) * (b - math.floor(b) + 1)
        a = math.floor(b)
        res += str(a)
    return res

s = (l + r) / 2
g = generate(s)
ss = trim(s)
sg = trim(g)
while ss != sg:
    if ss < sg:
        l = s
    else:
        r = s
    s = (l + r) / 2
    g = generate(s)
    ss = trim(s)
    sg = trim(g)

print(s, g, ss, sg, sep="\n")