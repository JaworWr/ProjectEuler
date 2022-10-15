import Base: ==, +, *

struct Fraction
    num :: Int
    denom :: Int

    function Fraction(num, denom)
        if denom == 0
            error("Denominator must not be 0")
        elseif denom < 0
            num = -num
            denom = -denom
        end
        new(num, denom)
    end
end

function simplify(a::Fraction)
    d = gcd(a.num, a.denom)
    Fraction(a.num / d, a.denom / d)
end

==(a::Fraction, b::Fraction) = a.num * b.denom == a.denom * b.num
+(a::Fraction, b::Fraction) = simplify(Fraction(
    a.num * b.denom + a.denom * b.num, 
    a.denom * b.denom,
))
*(a::Fraction, b::Fraction) = simplify(Fraction(
    a.num * b.num,
    a.denom * b.denom,
))

get_digit(x::T, i::Integer) where {T <: Integer} = parse(T, string(x)[i])

function remove_digit(x::T, d::Integer) where {T <: Integer}
    x = string(x)
    d = string(d)[1]
    if x[1] == d
        return Some(parse(T, x[2]))
    elseif x[2] == d
        return Some(parse(T, x[1]))
    else
        return nothing
    end
end

function extend_fraction(frac::Fraction)
    res = Set()
    for i in 1:2
        d = get_digit(frac.num, i)
        if d == 0
            continue
        end
        num1 = something(remove_digit(frac.num, d))
        denom1 = remove_digit(frac.denom, d)
        if !isnothing(denom1)
            denom1 = something(denom1)
            if denom1 == 0
                continue
            end
            frac1 = Fraction(num1, denom1)
            if frac1 == frac
                push!(res, frac1)
            end
        end
    end
    res
end

# println(Fraction(3, 5) == Fraction(6, 10))
# println(Fraction(3, 5) == Fraction(6, 11))
# println(get_digit(345, 2))
# println(remove_digit(34, 4))
# println(remove_digit(34, 5))
# println(extend_fraction(Fraction(49, 98)))
# println(extend_fraction(Fraction(49, 97)))
# println(extend_fraction(Fraction(30, 50)))

fractions = Vector{Fraction}()
for denom in 11:99
    for num in 10:denom-1
        frac = Fraction(num, denom)
        s = extend_fraction(frac)
        if !isempty(s)
            push!(fractions, frac)
        end
    end
end
println(fractions)

res = Fraction(1, 1)
for frac in fractions
    global res
    res *= frac
end
println(res)
