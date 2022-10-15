def is_lychrel(x):
    for i in range(50):
        xr = int(str(x)[::-1])
        if i > 0 and x == xr:
            return False
        x += xr
    return True


print(is_lychrel(47))
print(is_lychrel(349))
print(is_lychrel(4994))
print(is_lychrel(196))
print(sum(map(is_lychrel, range(10000))))
