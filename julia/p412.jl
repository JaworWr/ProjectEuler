# https://en.wikipedia.org/wiki/Hook_length_formula

M = 10000
N = 5000
MOD = 76543217

count = M * M - N * N
res = 1
for i in 1:count
    global res
    res = res * i % MOD
end

for i in 1:M
    global res
    if i <= N
        for j in 1:M-N
            hooklen = j + i - 1
            res = res * invmod(hooklen, MOD) % MOD
        end
    else
        for j in N+1:M
            hooklen = j + i - 1
            res = res * invmod(hooklen, MOD) % MOD
        end
        for j in 1:N
            hooklen = j + i - N - 1
            res = res * invmod(hooklen, MOD) % MOD
        end
    end
end
println(res)