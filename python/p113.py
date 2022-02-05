def choice(n, k):
    assert n >= k
    k = min(k, n - k)
    num = 1
    denom = 1
    for i in range(n - k + 1, n + 1):
        num *= i
    for i in range(1, k + 1):
        denom *= i
    return num // denom

DIGITS = 100
inc = choice(DIGITS + 9, DIGITS) - 1
dec = choice(DIGITS + 10, DIGITS) - DIGITS - 1
incdec = 9 * DIGITS
print(inc, dec, incdec, inc + dec - incdec)
