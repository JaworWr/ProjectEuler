setprecision(150, base=10)


function sod(x::BigFloat)::Int
    if x - floor(x) < 1e-8
        return 0
    end
    p = floor(log10(x)) + 1
    x = x * BigFloat(10)^(100 - p)
    x = BigInt(floor(x))
    s = 0
    while x > 0
        s += mod(x, 10)
        x = div(x, 10)
    end
    s
end

sums = map(1:100) do x
    x = BigFloat(x)
    sod(sqrt(x))
end
println(sum(sums))
