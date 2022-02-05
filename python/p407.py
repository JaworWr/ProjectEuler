def best(x):
    m = 0
    for i in range(x):
        if i * i % x == i:
            m = i
    return m
