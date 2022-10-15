triangle_numbers = collect(1:26*50)
triangle_numbers = div.(triangle_numbers .* (triangle_numbers .+ 1), 2)
triangle_numbers = Set(triangle_numbers)

function word_value(word::AbstractString)
    res = 0
    for letter in word
        res += Int(letter) - Int('A') + 1
    end
    res
end

c = 0
open("p42.txt") do f
    global words
    words = readline(f)
end
words = split(words, ',')
words = map(w -> w[2:end-1], words)
for w in words
    if word_value(w) in triangle_numbers
        global c
        c += 1
    end
end
println(c)
