#include <iostream>
#include <vector>

using namespace std;

const int MAX = 1000000;

int spf[MAX+5];
bool is_composite[MAX+5];
vector<int> primes;

void sieve()
{
    is_composite[0] = is_composite[1] = true;
    for (int i = 2; i <= MAX; i++) {
        if (!is_composite[i]) {
            primes.push_back(i);
            spf[i] = i;
        }
        for (int p : primes) {
            if (i * p > MAX || p > spf[i]) {
                break;
            }
            is_composite[i * p] = true;
            spf[i * p] = p;
        }
    }
}

int totient(int x)
{
    int spf_last = 1;
    int result = x;
    while (x > 1) {
        if (spf[x] != spf_last) {
            result = result / spf[x] * (spf[x] - 1);
            spf_last = spf[x];
        }
        x /= spf[x];
    }
    return result;
}

int main()
{
    sieve();
    long long s = 0;
    for (int i = 2; i <= MAX; i++) {
        s += totient(i);
    }
    cout << s << endl;
}
