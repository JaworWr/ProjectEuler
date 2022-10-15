import numpy as np

x = np.array([[0, 0, 0, 0], [0, 1, 0, 1]])
while np.max(x[0, :]) < 10**12:
    y = np.empty_like(x)
    y[0, :2] = 3*x[0, :2] + 4*x[1, :2] - 3
    y[1, :2] = 2*x[0, :2] + 3*x[1, :2] - 2
    y[0, 2:] = 3*x[0, 2:] - 4*x[1, 2:] + 1
    y[1, 2:] = -2*x[0, 2:] + 3*x[1, 2:]
    x = y

print(x)