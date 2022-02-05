import itertools

def contains(cube, c):
    if c in [6, 9]:
        return 6 in cube or 9 in cube
    return c in cube

SQUARES = [1, 4, 9, 16, 25, 36, 49, 64, 81]

def validate(c1, c2):
    for s in SQUARES:
        a, b = divmod(s, 10)
        ok = (contains(c1, a) and contains(c2, b)) or (contains(c2, a) and contains(c1, b))
        if not ok:
            return False
    return True

print(validate({0, 5, 6, 7, 8, 9}, {1, 2, 3, 4, 5, 7}))
print(validate({0, 5, 6, 7, 8, 9}, {1, 2, 3, 4, 6, 7}))
print(validate({0, 5, 6, 7, 8, 9}, {1, 2, 3, 4, 8, 9}))

digits = list(range(10))
cube_settings = [set(t) for t in itertools.combinations(digits, 6)]
all_settings = list(itertools.product(cube_settings, repeat=2))
print(len(all_settings))

res = 0
for c1, c2 in all_settings:
    if c1 == c2:
        res += 2 * validate(c1, c2)
    else:
        res += validate(c1, c2)
print(res // 2)