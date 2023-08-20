from fractions import Fraction


seq = [2, 1]
for i in range(1, 50):
    seq += [2 * i, 1, 1]


def convergent(i):
    c = Fraction(seq[i], 1)
    for j in range(i - 1, -1, -1):
        c = seq[j] + 1 / c
    return c


def sod(x):
    return sum(map(int, str(x)))

print(convergent(9))
print(sod(convergent(9).numerator))

print(sod(convergent(99).numerator))
