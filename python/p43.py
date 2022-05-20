from itertools import permutations
from math import factorial
from tqdm import tqdm


def check_number(x):
    def substr_at_divisible(d, n):
        d -= 1
        v = int(x[d:d+3])
        return v % n == 0
    
    args = zip(
        [2, 3, 4, 5, 6, 7, 8],
        [2, 3, 5, 7, 11, 13, 17]
    )
    return all(substr_at_divisible(d, n) for d, n in args)


candidates = permutations("0123456789")
candidates = tqdm(candidates, total=factorial(10))
candidates = filter(lambda x: x[0] != "0", candidates)
candidates = map(lambda x: "".join(x), candidates)
candidates = filter(check_number, candidates)
candidates = map(int, candidates)
result = sum(candidates)
print(result)
