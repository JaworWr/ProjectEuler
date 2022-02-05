#include <iostream>
#include <vector>
#include <cmath>

using namespace std;
const int MAX = 1000000;

int mdrs[MAX + 5];

int drs(int x)
{
    int tmp;
    while (x >= 10) {
        tmp = 0;
        while (x > 0) {
            tmp += x % 10;
            x /= 10;
        }
        x = tmp;
    }
    return x;
}

inline vector<int> factors(int x)
{
    vector<int> results;
    int i, step;
    if (x % 2) {
        i = 3;
        step = 2;
    }
    else {
        i = 2;
        step = 1;
    }
    int m = round(sqrt((double) x));
    for(; i <= m; i += step) {
        if (x % i == 0) {
            results.push_back(i);
        }
    }
    return results;
}

int get_mdrs(int x)
{
    if (mdrs[x] == 0) {
        mdrs[x] = drs(x);
        for (int f : factors(x)) {
            int s = get_mdrs(f) + get_mdrs(x / f);
            if (s > mdrs[x]) {
                mdrs[x] = s;
            }
        }
    }
    return mdrs[x];
}

int main()
{
    long long s = 0;
    for (int i = 2; i < 1000000; i++) {
        s += get_mdrs(i);
    }
    cout << s << endl;
}
