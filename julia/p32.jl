using ProgressBars

function permutations(v::Vector)
    if length(v) <= 1
        return [v]
    end

    perms = permutations(v[2:end])
    result = []
    for p in perms
        push!(result, vcat(v[1], p))
        for i in 1:length(p) - 1
            p1 = vcat(p[1:i], v[1], p[i+1:end])
            push!(result, p1)
        end
        push!(result, vcat(p, v[1]))
    end
    result
end

function partitions(v::Vector)
    results = []
    for i in 1:length(v) - 2
        for j in i + 1:length(v) - 1
            push!(results, (v[1:i], v[i+1:j], v[j+1:end]))
        end
    end
    results
end

function from_digits(v::Vector{Int}, base::Int = 10)
    result = 0
    for x in v
        result = result * base + x
    end
    result
end

println(partitions(collect(1:6)))

products = Set{Int}()
for p in tqdm(permutations(collect(1:9)))
    for (a, b, c) in partitions(p)
        a = from_digits(a)
        b = from_digits(b)
        c = from_digits(c)
        if a * b == c
            push!(products, c)
        end
    end
end
println(7254 in products)
println(sum(products))
