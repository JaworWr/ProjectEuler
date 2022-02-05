#include <iostream>
#include <vector>
#include <utility>
#include <algorithm>

using namespace std;
typedef long long LL;
const int MAX = 100000;

bool is_composite[MAX+5];
int spf[MAX+5];
vector<int> primes;

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

int rad(int x)
{
    int last = 1;
    int res = 1;
    while (x > 1) {
        if (spf[x] != last) {
            res *= spf[x];
            last = spf[x];
        }
        x /= spf[x];
    }
    return res;
}

int main()
{
    sieve();
    vector<pair<int, int>> v;
    for (int i = 1; i <= MAX; i++) {
        v.push_back(make_pair(rad(i), i));
    }
    sort(v.begin(), v.end());
    cout << v[10000 - 1].second << endl;
}
