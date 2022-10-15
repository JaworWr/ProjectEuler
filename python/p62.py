from collections import defaultdict


cubes = defaultdict(list)

for n in range(10000):
    c = n**3
    cubes[len(str(c))].append(c)


def is_permutation(x, y):
    return sorted(str(x)) == sorted(str(y))


def best():
    for l in cubes.values():
        for x in l:
            c = 0
            for y in l:
                if is_permutation(x, y):
                    c += 1
            if c == 5:
                return x

print(best())
