def p(d, n):
    d = str(d)
    i = 0
    j = 0
    k = "1"
    while j < n:
        i += 1
        k = str(2 * int(k))
        if k.startswith(d):
            j += 1
        k = k[:10*len(d)]
    return i


print(p(123, 678910))
