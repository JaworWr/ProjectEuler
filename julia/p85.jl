choices(x) = div((x + 1) * x, 2)
choices(x, y) = choices(x) * choices(y)

N = 2000000

@assert choices(100, 100) > N
@assert choices(3, 2) == 18

best_d = 9999999999
best_area = 0

for w in 1:100
    for h in 1:100
        d = abs(choices(w, h) - N)
        if d < best_d
            global best_d, best_area
            best_d = d
            best_area = w * h
        end
    end
end
println(best_d, " ", best_area)
