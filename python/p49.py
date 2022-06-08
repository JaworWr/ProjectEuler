from utils import is_prime


def is_permutation(x, y):
    return sorted(str(x)) == sorted(str(y))


def check(a, b, c):
    if not all(is_prime(x) for x in [a, b, c]):
        return False
    if not is_permutation(a, b):
        return False
    if not is_permutation(a, c):
        return False
    return True


for a in range(1000, 9998):
    for b in range(a + 1, 9999):
        c = b + (b - a)
        if check(a, b, c):
            print(a, b, c)
