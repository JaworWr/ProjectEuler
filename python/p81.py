import numpy as np


def read_matrix(path):
    res = []
    with open(path) as f:
        for line in f:
            res.append([int(x) for x in line.strip().split(",")])
    return np.array(res)


def solve(matrix: np.ndarray):
    dp = np.zeros_like(matrix)
    dp[0, :] = np.cumsum(matrix[0, :])
    dp[:, 0] = np.cumsum(matrix[:, 0])
    h, w = matrix.shape
    for i in range(1, h):
        for j in range(1, w):
            dp[i, j] = min(dp[i - 1, j], dp[i, j - 1]) + matrix[i, j]
    return dp[h - 1, w - 1]


matrix = np.array([
    [131, 673, 234, 103,  18],
    [201,  96, 342, 965, 150],
    [630, 803, 746, 422, 111],
    [537, 699, 497, 121, 956],
    [805, 732, 524,  37, 331],
])
print(solve(matrix))

matrix = read_matrix("p81.txt")
print(solve(matrix))
