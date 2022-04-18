from re import L


def iter_digits():
    n = 1
    while True:
        yield from map(int, str(n))
        n += 1


r = 1
for i, d in enumerate(iter_digits()):
    if i in {0, 9, 99, 999, 9999, 99999, 999999}:
        r *= d
    if i == 999999:
        break
print(r)
