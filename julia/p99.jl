values = open("p99.txt") do f
    map(readlines(f)) do line
        a, b = split(line, ',')
        (parse(Int, a), parse(Int, b))
    end
end

log_value((a, b)) = log(Float64(a)) * b
println(log_value(values[1]))
println(log_value(values[2]))

res = map(log_value, values) |> argmax
println(res)
