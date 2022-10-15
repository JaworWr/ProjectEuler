include("utils.jl")
using .FixArgs

text = open("p22.txt") do f
    read(f, String)
end

names = map(name -> name[2:end-1], split(text, ','))
sort!(names)

const base = Int('A') - 1
name_value(name::AbstractString) = sum(Int(c) - base for c in name)

println(name_value("COLIN"))

name_values = [name_value(name) for name in names]

value((idx, v)) = idx * v
ans = name_values |> enumerate |> FixFirst(map, value) |> sum
println(ans)