from fractions import Fraction


f = 1 + Fraction(1, 2)
fs = []
for i in range(1000):
    fs.append(f)
    f = 1 + 1 / (1 + f)

def is_cool(f: Fraction) -> bool:
    n = len(str(f.numerator))
    m = len(str(f.denominator))
    return n > m

result = sum(map(is_cool, fs))
print(result)
