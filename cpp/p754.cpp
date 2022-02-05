#include <iostream>
#include <vector>
#include <chrono>

using namespace std;
typedef long long LL;
const int MAX = 100000000;
const LL MOD = 1000000007ll;

bool is_composite[MAX+5];
int spf[MAX+5];
vector<int> primes;

LL fast_pow(LL a, LL b, LL m)
{
    LL res = 1;
    while (b > 0) {
        if (b % 2) {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    return res;
}

void sieve()
{
    is_composite[0] = is_composite[1] = true;
    for (int i = 2; i <= MAX; i++) {
        if (!is_composite[i]) {
            spf[i] = i;
            primes.push_back(i);
        }
        for (size_t j = 0; j < primes.size() && i * primes[j] <= MAX && primes[j] <= spf[i]; j++) {
            is_composite[i * primes[j]] = true;
            spf[i * primes[j]] = primes[j];
        }
    }
}

inline vector<int> upf(int x)
{
    vector<int> res;
    while (x > 1) {
        if (res.empty() || res.back() != spf[x]) {
            res.push_back(spf[x]);
        }
        x /= spf[x];
    }
    return res;
}

vector<LL> multipliers(const vector<int>& primes)
{
    vector<LL> res;
    vector<LL> temp;
    for (LL p : primes) {
        temp.push_back(p);
        for (LL q : res) {
            temp.push_back(-p * q);
        }
        res.insert(res.end(), temp.begin(), temp.end());
        temp.clear();
    }
    return res;
}

LL totient(int x, const vector<int>& primes)
{
    LL res = x;
    for (int p : primes) {
        res -= res / p;
    }
    return res;
}

LL n_occ(int x, int max = MAX)
{
    auto prime_factors = upf(x);
    LL phi = totient(x, prime_factors);
    LL res = 0;
    for (int m : multipliers(prime_factors)) {
        res += max / m;
    }
    return max - res - phi;
}

int main()
{
    auto start_time = chrono::steady_clock::now();
    sieve();
    auto end_time = chrono::steady_clock::now();
    auto time_ms = chrono::duration_cast<chrono::milliseconds>(end_time - start_time);
    cout << "Sieve time: " << time_ms.count() << "ms" << endl;

    int g = 1;
    int k = MAX;
    start_time = chrono::steady_clock::now();
    for (int i = 2; i <= k; i++) {
        if (i % 1000000 == 0) {
            end_time = chrono::steady_clock::now();
            time_ms = chrono::duration_cast<chrono::milliseconds>(end_time - start_time);
            cout << i << ": " << (float)time_ms.count() / 1000 << " seconds" << endl;
        }
        g = (g * fast_pow(i, n_occ(i, k), MOD)) % MOD;
    }
    cout << g << endl;
}