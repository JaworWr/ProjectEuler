#include <iostream>
#include <cmath>

using namespace std;

int divisor_count(int x)
{
    int step, c = 0;
    if (x % 2) {
        step = 2;
    }
    else {
        step = 1;
    }
    int m = (int) round(sqrt((double) x));
    for (int i = 1; i <= m; i += step) {
        if (x % i == 0) {
            c += 1 + (i < m || i * i != x);
        }
    }
    return c;
}


int main() {
    int x = 1, d = 2;
    while (divisor_count(x) <= 500) {
        x += d;
        d++;
    }
    cout << x << endl;
}
