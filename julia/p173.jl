M = 1000000
ways = zeros(Int, M)

n = div(M, 4) + 1
c = 0
for i in 1:n
    for j in i-2:-2:1
        global c
        x = i^2 - j^2
        if x > M
            break
        end
        c += 1
    end
end

println(c)
