def load_triangle(path):
    all_numbers = []
    with open(path) as f:
        for line in f:
            numbers = [int(x) for x in line.strip().split()]
            all_numbers.append(numbers)
    return all_numbers

def solve(triangle):
    best_paths = triangle[0]
    for row in triangle[1:]:
        new_best_paths = []
        for i, x in enumerate(row):
            b = 0
            if i < len(best_paths):
                b = best_paths[i] + x
            if i > 0:
                b = max(b, best_paths[i - 1] + x)
            new_best_paths.append(b)
        best_paths = new_best_paths

    return max(best_paths)


t18 = load_triangle("p18.txt")
print("p18", solve(t18))

t67 = load_triangle("p67.txt")
print("p67", solve(t67))