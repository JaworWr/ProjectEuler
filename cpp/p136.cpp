#include <iostream>

using namespace std;
typedef long long LL;

const LL N = 50000000;
const LL m = (N + 1) / 4;
LL counts[N];

int main()
{
    for (LL a = 1; a < 3 * m; a++) {
        LL min_k = a / 3 + 1;
        for (LL k = min_k; k < m; k++) {
            LL r = 3*k*k + 2*a*k - a*a;
            if (r < N) {
                counts[r]++;
            }
            else {
                break;
            }
        }
    }

    int s = 0;
    for (int i = 0; i < N; i++) {
        s += (counts[i] == 1);
    }
    cout << s << endl;
}
