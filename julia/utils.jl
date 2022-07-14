module FixArgs

struct FixFirst{F, Args} <: Function
    f::F
    args::Args

    FixFirst(f::F, args::Args) where {F, Args <: Tuple} = new{F, Args}(f, args)
    FixFirst(f::Type{F}, args::Args) where {F, Args <: Tuple} = new{Type{F}, Args}(f, args)
end

FixFirst(f, args...) = FixFirst(f, args)
(ff::FixFirst)(x) = ff.f(ff.args..., x)
export FixFirst

struct FixLast{F, Args} <: Function
    f::F
    args::Args

    FixLast(f::F, args::Args) where {F, Args <: Tuple} = new{F, Args}(f, args)
    FixLast(f::Type{F}, args::Args) where {F, Args <: Tuple} = new{Type{F}, Args}(f, args)
end

FixLast(f, args...) = FixLast(f, args)
(ff::FixLast)(x) = ff.f(x, ff.args...)
export FixLast

end

module Utils

function factors(x::T) where {T<:Integer}
    if x == 0 
        return []
    end

    res::Vector{T} = []
    m = floor(sqrt(x))
    for i in 1:m
        if x % i == 0
            push!(res, i)
            j = div(x, i)
            if j != i
                push!(res, j)
            end
        end
    end
    res
end
export factors

function is_prime(x::Integer)
    if x < 2
        return false
    end

    m = floor(sqrt(x))
    for i in 2:m
        if x % i == 0
            return false
        end
    end
    true
end
export is_prime

end

module ModularArithmetic

legendre(n::Integer, p::Integer) = powermod(n, (p-1) ÷ 2, p)
export legendre


function tonelli_shanks(n::Integer, p::Integer)
    if n == 0
        return 0
    end
    if legendre(n, p) != 1
        throw(DomainError((n, p), "square root does not exist"))
    end

    q = p - 1
    s = 0
    while q % 2 == 0
        q ÷= 2
        s += 1
    end

    z = 0
    for i in 2:p-1
        if legendre(i, p) != 1
            z = i
            break
        end
    end

    m = s
    c = powermod(z, q, p)
    t = powermod(n, q, p)
    r = powermod(n, (q+1) ÷ 2, p)
    while true
        if t == 0
            return 0
        elseif t == 1
            r = min(r, p - r)
            return r
        end
        i = 0
        tmp = t
        while tmp != 1
            tmp = powermod(tmp, 2, p)
            i += 1
        end
        b = c
        for _ in 1:m-i-1
            b = powermod(b, 2, p)
        end
        m = i
        c = powermod(b, 2, p)
        t = t * c % p
        r = r * b % p
    end
end
export tonelli_shanks

end
