from collections import defaultdict


def get_numbers(f):
    res = []
    for n in range(10000):
        x = f(n)
        if x < 1000:
            continue
        if x > 9999:
            break
        res.append(x)
    return res

candidates = {
    3: get_numbers(lambda n: n * (n+1) // 2),
    4: get_numbers(lambda n: n**2),
    5: get_numbers(lambda n: n * (3*n-1) // 2),
    6: get_numbers(lambda n: n * (2*n-1)),
    7: get_numbers(lambda n: n * (5*n-3) // 2),
    8: get_numbers(lambda n: n * (3*n-2)),
}

K = 8

number_classes = defaultdict(list)
for i in range(3, K+1):
    for x in candidates[i]:
        number_classes[x].append(i)

all_candidates = list(number_classes.keys())
print(len(all_candidates))

edges = defaultdict(set)
for c1 in all_candidates:
    for c2 in all_candidates:
        if c1 == c2:
            continue
        if c1 % 100 == c2 // 100:
            edges[c1].add(c2)

def dfs():
    def aux(value, classes, first, last_c):
        if classes == set(range(3, K+1)):
            if first in edges[value]:
                yield [(value, last_c)]
        else:
            for x in edges[value]:
                for c in number_classes[x]:
                    if c in classes:
                        continue
                    for l in aux(x, classes | {c}, first, c):
                        yield [(value, last_c)] + l

    for x in candidates[3]:
        yield from aux(x, {3}, x, 3)

for solution in dfs():
    print(solution, sum(x[0] for x in solution))