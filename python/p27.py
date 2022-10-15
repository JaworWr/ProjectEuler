from tqdm import trange
from utils import is_prime


best = 0
best_a = 0
best_b = 0

for a in trange(-999, 1000):
    for b in range(-1000, 1001):
        for n in range(10000):
            if not is_prime(n**2 + a*n + b):
                break
            if n > best:
                best = n
                best_a = a
                best_b = b

print(best, best_a, best_b, best_a * best_b)
