const MAX = 10^7

function get_digits(x::T)::Vector{T} where {T <: Integer}
    digits = string(x) |> collect
    digits = map(digits) do c
        parse(T, c)
    end
    collect(digits)
end

is_ok(x::Int) = x == map(factorial, digits(x)) |> sum

println(is_ok(123))
println(is_ok(145))

res = filter(is_ok, collect(3:MAX)) |> sum
println(res)
