#include <vector>
#include <iostream>

using namespace std;
using ull = unsigned long long;

const ull MAX = 1000000;

int main()
{
    // h - last required value of H(k)
    ull k = 2, n = 2, n3 = 8, g = 1, h = 3, h0_idx = 0;
    // h0 - auxiliary sequence of H(k) reqruired to calculate h
    vector<ull> h0 = {1};
    ull s = 1;
    while (n < MAX) {
        // calculate H(k+1)
        k++;
        // G(n) == k iff H(k-1) < n <= H(k); H(k) = h[k-1]
        // knowing this, update g as necessary (g = G(k+1))
        if (h0.back() < k) {
            g++;
            if (h0_idx == 0) {
                h0_idx = 1;
                h0.push_back(3);
            }
            else {
                if (h0[h0_idx] < h0.size() + 1) h0_idx++;
                h0.push_back(h0.back() + h0_idx + 1);
            }
        }
        h += g;
        // update all n such that n^3 <= H(k+1)
        while(n < MAX && n3 <= h) {
            s += k;
            n++;
            n3 = n * n * n;
        }
    }
    cout << s << endl;
}