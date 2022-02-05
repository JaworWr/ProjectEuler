import numpy as np

s = 0
for i in range(100):
    v = np.float64(1) - (np.float64(1) - np.float64(0.5) ** i) ** 32
    s += v
    print(v)
print(f"Result: {s:.10f}")
