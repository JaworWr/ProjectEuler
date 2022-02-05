#include <iostream>
#include <vector>
#include <unordered_set>

using namespace std;
typedef long long LL;
const int PMAX = 500000;
const int MAX = 50000000;

bool is_composite[PMAX+5];
int spf[PMAX+5];
vector<int> primes;

void sieve()
{
    is_composite[0] = is_composite[1] = true;
    for (int i = 2; i <= PMAX; i++) {
        if (!is_composite[i]) {
            spf[i] = i;
            primes.push_back(i);
        }
        for (size_t j = 0; j < primes.size() && i * primes[j] <= PMAX && primes[j] <= spf[i]; j++) {
            is_composite[i * primes[j]] = true;
            spf[i * primes[j]] = primes[j];
        }
    }
}

int main()
{
    sieve();
    unordered_set<int> found;
    for (size_t ia = 0; ia < primes.size(); ia++) {
        int a = primes[ia] * primes[ia];
        if (a >= MAX) {
            break;
        }
        for (size_t ib = 0; ib < primes.size(); ib++) {
            int b = a + primes[ib] * primes[ib] * primes[ib];
            if (b >= MAX) {
                break;
            }
            for (size_t ic = 0; ic < primes.size(); ic++) {
                int c = b + primes[ic] * primes[ic] * primes[ic] * primes[ic];
                if (c >= MAX) {
                    break;
                }
                else {
                    found.insert(c);
                }
            }
        }
    }
    cout << found.size() << endl;
}
