from distutils.util import change_root
from math import factorial
from tqdm import tqdm


def step(x):
    digits = map(int, str(x))
    return sum(map(factorial, digits))


def chain_length(x):
    s = {x}
    y = step(x)
    while y not in s:
        s.add(y)
        y = step(y)
    return len(s)


print(step(169))
print(chain_length(169))
print(chain_length(69))
print(chain_length(78))


numbers = tqdm(range(1000000))
ok_numbers = filter(lambda x: chain_length(x) == 60, numbers)
print(sum(1 for _ in ok_numbers))