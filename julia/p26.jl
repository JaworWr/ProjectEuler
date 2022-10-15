function to_decimal(denom::Int)
    num = 10
    res = Int[]
    past_nums = Dict()
    i = 1
    cycle_start = nothing
    while num != 0
        if haskey(past_nums, num)
            cycle_start = past_nums[num]
            break
        end
        past_nums[num] = i
        i += 1

        if num < denom
            num *= 10
            push!(res, 0)
        else
            digit, num = divrem(num, denom)
            push!(res, digit)
            num *= 10
        end
    end
    if cycle_start === nothing
        res, Int[]
    else
        res[1:cycle_start - 1], res[cycle_start:end]
    end
end

best = (1, 0)
for i in 2:999
    global best
    base, cycle = to_decimal(i)
    score = length(cycle)
    if score > best[1]
        best = (i, score)
    end
end
println(join(best, ' '))
