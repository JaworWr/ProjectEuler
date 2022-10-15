include("utils.jl")
using .FixArgs

function check(x::Int, power::Int)
    digits = [parse(Int, d) for d in string(x)]
    sum(map(d -> d^power, digits)) == x
end

res = 10:10000 |> FixFirst(filter, x -> check(x, 4)) |> sum
println(res)
# TODO: prove that 1000000 is enough
res = 10:1000000 |> FixFirst(filter, x -> check(x, 5)) |> sum
println(res)