include("utils.jl")
using .Utils
using Base.Iterators

function is_abundant(x::Integer)
    f = factors(x)
    sort!(f)
    s = sum(f[1:end-1])
    s > x
end

const candidates = 1:28123
const abundant = filter(is_abundant, candidates)
println(abundant[1:10])

function check(x::Integer)
    c = Set(takewhile(y -> y <= x, abundant))
    for y in c
        if x - y in c
            return false
        end
    end
    true
end

ans = sum(filter(check, candidates))
println(ans)
