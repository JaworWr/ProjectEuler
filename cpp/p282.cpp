#include <iostream>
#include <utility>
#include <unordered_map>
#include <cmath>
#include <stack>
#include <boost/functional/hash.hpp>

using namespace std;
typedef long long LL;
const LL MOD = 1475789056LL;

unordered_map<pair<LL, LL>, LL, boost::hash<pair<LL, LL>>> cache;

void calc_ackermann(LL m, LL n)
{
    stack<pair<LL, LL>> st;
    st.push(make_pair(m, n))
    while (!st.empty()) {
        auto p = st.top();
        m = p.first;
        n = p.second;
        if (cache.find(make_pair(m, n)) != cache.end()) {
            st.pop();
        }
        else if (m == 0) {
            cache[make_pair(m, n)] = n + 1;
        }
        else if (n == 0) {
            auto it = cache.find(make_pair(m - 1, 1));
            if (it == cache.end()) {
                st.push(make_pair(m - 1, 1));
            }
            else {
                cache[make_pair(m, n)] = it->second;
                st.pop();
            }
        }
        else {
            auto it1 = cache.find(make_pair(m, n - 1));
            if (it1 == cache.end()) {
                st.push(make_pair(m, n - 1));
                continue;
            }
            auto it2 = cache.find(make_pair(m - 1, it1->second));
            if (it2 == cache.end()) {
                st.push(make_pair(m - 1, it1->second));
                continue;
            }
            cache[make_pair(m, n)] = it2->second;
        }
    }
}

LL ackermann(LL m, LL n)
{
    calc_ackermann(m, n);
    return cache[make_pair(m, n)];
}

int main()
{
    cout << ackermann(1, 0) << endl << ackermann(2, 2) << endl << ackermann(3, 4) << endl;
    LL s = 0;
    for (LL n = 0; n <= 4; n++) {
        s = (s + ackermann(n, n)) % MOD;
    }
    cout << s << endl;
}
