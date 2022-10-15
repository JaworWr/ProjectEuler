# to run: julia p81_82_83.jl <problem_name> <matrix_file>

mutable struct PriorityQueue{P, V}
    data::Vector{Tuple{P, V}}

    function PriorityQueue{P, V}() where {P, V}
        data = Vector{Tuple{P, V}}()
        new(data)
    end
end

Base.isempty(q::PriorityQueue{P, V}) where {P, V} = isempty(q.data)
Base.length(q::PriorityQueue{P, V}) where {P, V} = length(q.data)

function swap!(a, i::Int, j::Int)
    tmp = a[i]
    a[i] = a[j]
    a[j] = tmp
end

function move_up!(q::PriorityQueue{P, V}, idx::Int) where {P, V}
    while idx > 1
        idx1 = div(idx, 2)
        if q.data[idx][1] < q.data[idx1][1]
            swap!(q.data, idx, idx1)
        else
            break
        end
        idx = idx1
    end
end

function move_down!(q::PriorityQueue{P, V}, idx::Int) where {P, V}
    while 2 * idx <= length(q)
        k = idx
        if q.data[k][1] > q.data[2 * idx][1]
            k = 2 * idx
        end
        if 2*idx + 1 <= length(q) && q.data[k][1] > q.data[2*idx + 1][1]
            k = 2*idx + 1
        end
        if idx == k
            break
        end
        swap!(q.data, k, idx)
        idx = k
    end
end

function Base.push!(q::PriorityQueue{P, V}, p::P, v::V) where {P, V}
    push!(q, (p, v))
end

function Base.push!(q::PriorityQueue{P, V}, x::Tuple{P, V}) where {P, V}
    push!(q.data, x)
    move_up!(q, length(q))
end

function Base.pop!(q::PriorityQueue{P, V})::Tuple{P, V} where {P, V}
    swap!(q.data, 1, length(q))
    res = pop!(q.data)
    move_down!(q, 1)
    res
end

NeighborMap{T} = Dict{T, Vector{Tuple{Int, T}}}

function dijkstra(graph::NeighborMap{T}, start::T, is_end::Function) where T
    q = PriorityQueue{Int, T}()
    push!(q, 0, start)
    visited = Set()

    while !isempty(q)
        (d, x) = pop!(q)
        if x in visited
            continue
        end
        if is_end(x...)
            return d
        end
        push!(visited, x)
        for (e, n) in graph[x]
            if n in visited
                continue
            end
            push!(q, d + e, n)
        end
    end
    -1
end

matrix = open(ARGS[2]) do f
    m = []
    map(eachline(f)) do line
        row = map(x -> parse(Int, x), split(line, ','))
        push!(m, collect(row))
    end
    mapreduce(permutedims, vcat, m)
end
n, m = size(matrix)

function get_neighbors(matrix, neighbors_cb, start_vertices)::NeighborMap{Tuple{Int, Int}}
    neighbors = Dict()
    ns = start_vertices
    ns = map(x -> (matrix[x...], x), ns)
    neighbors[(0, 0)] = ns
    for i in 1:n
        for j in 1:m
            ns = neighbors_cb(i, j)
            ns = filter(x -> 1 <= x[1] <= n && 1 <= x[2] <= m, ns)
            ns = map(x -> (matrix[x...], x), ns)
            neighbors[(i, j)] = ns
        end
    end
    neighbors
end

neighbors_p81(i, j) = [(i+1, j), (i, j+1)]
neighbors_p82(i, j) = [(i+1, j), (i-1, j), (i, j+1)]
neighbors_p83(i, j) = [(i+1, j), (i-1, j), (i, j-1), (i, j+1)]

starts = [(1, 1)]
starts_p82 = [(i, 1) for i in 1:n]

is_end(i, j) = i == n && j == m
is_end_p82(i, j) = j == m

neighbor_data = Dict([
    ("p81", (neighbors_p81, starts, is_end)),
    ("p82", (neighbors_p82, starts_p82, is_end_p82)),
    ("p83", (neighbors_p83, starts, is_end)),
])

neighbors_cb, start_vertices, end_cb = neighbor_data[ARGS[1]]

neighbors = get_neighbors(matrix, neighbors_cb, start_vertices)

println(dijkstra(neighbors, (0, 0), end_cb))
