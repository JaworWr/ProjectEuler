M = 1000000
ways = zeros(Int, M)

n = div(M, 4) + 1

for i in 1:n
    for j in i-2:-2:1
        x = i^2 - j^2
        if x > M
            break
        end
        ways[x] += 1
    end
end

println(ways[8])
println(ways[32])

Mt = 20
totals = zeros(Int, Mt)
for x in ways
    if x > 0 && x <= Mt
        totals[x] += 1
    end
end
println(totals[2])
println(totals[15])
println(sum(totals[1:10]))
