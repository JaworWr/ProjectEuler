from tqdm import tqdm

ways = [1]
it = tqdm(range(1, 1000000))
for i in it:
    ways.append(0)
    k = 1
    l = k * (3 * k - 1) // 2
    while l <= i:
        ways[-1] += ways[i - l] * (-1 if k % 2 == 0 else 1)
        if k > 0:
            k = -k
        else:
            k = -k + 1
        l = k * (3 * k - 1) // 2
    if ways[-1] % 1000000 == 0:
        print(i, ways[-1])
        break
