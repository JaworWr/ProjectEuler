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
