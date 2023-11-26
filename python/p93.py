from fractions import Fraction
import itertools
import multiprocessing
from tqdm import tqdm


def splits(xs: list[int]):
    for i in range(len(xs) // 2):
        i += 1
        for ys in itertools.combinations(xs, i):
            ys = set(ys)
            zs = xs - ys
            yield ys, zs


def all_values(xs: set[Fraction]) -> set[Fraction]:
    if len(xs) == 1:
        return xs
    
    results = set()
    for xl, xr in splits(xs):
        yl = all_values(xl)
        yl = yl | {-x for x in yl}
        yr = all_values(xr)
        yr = yr | {-x for x in yr}
        for vl, vr in itertools.product(yl, yr):
            results.add(vl + vr)
            results.add(vl * vr)
            if vr != 0:
                results.add(vl / vr)
            if vl != 0:
                results.add(vr / vl)
    return results
        

def get_values(xs: list[int]) -> set[int]:
    xs = {Fraction(x) for x in xs}
    res = all_values(xs)
    res = {abs(x) for x in res if x.denominator == 1}
    return res


def longest_sequence(xs: list[int]) -> int:
    res = get_values(xs)
    for i in range(1, 99999):
        if i not in res:
            return i - 1
        

candidates = list(map(list, itertools.combinations(range(10), 4)))
with multiprocessing.Pool(4) as pool:
    longest = pool.imap(longest_sequence, candidates)
    longest = list(tqdm(longest, total=len(candidates)))

max_idx = longest.index(max(longest))
print(candidates[max_idx], longest[max_idx])
