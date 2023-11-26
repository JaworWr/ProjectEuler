mapping = {
    "M": 1000,
    "CM": 900,
    "D": 500,
    "CD": 400,
    "C": 100,
    "XC": 90,
    "L": 50,
    "XL": 40,
    "X": 10,
    "IX": 9,
    "V": 5,
    "IV": 4,
    "I": 1,
}

prefixes = sorted(mapping.keys(), key=lambda x: -len(x))


def from_roman(x: str) -> int:
    v = 0
    while x:
        for pref in prefixes:
            if x.startswith(pref):
                x = x.removeprefix(pref)
                v += mapping[pref]
                break
    return v


def to_roman(x: int) -> str:
    s = []
    while x > 0:
        for pref, v in mapping.items():
            if v <= x:
                x -= v
                s.append(pref)
                break
    return "".join(s)


d = 0
with open("p89.txt") as f:
    for line in f:
        line = line.strip()
        val = from_roman(line)
        opt = to_roman(val)
        d += len(line) - len(opt)
print(d)
