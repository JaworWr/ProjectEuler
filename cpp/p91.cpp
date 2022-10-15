#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int dist_sq(int x1, int y1, int x2, int y2)
{
    return (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
}

bool is_right(int x1, int y1, int x2, int y2)
{
    vector<int> dists(3);
    dists[0] = dist_sq(x1, y1, 0, 0);
    dists[1] = dist_sq(x2, y2, 0, 0);
    dists[2] = dist_sq(x1, y1, x2, y2);
    sort(dists.begin(), dists.end());
    return dists[0] + dists[1] == dists[2];
}

const int MAX = 50;

int main()
{
    int res = 0;
    for (int x1 = 0; x1 <= MAX; x1++) {
        for (int y1 = 0; y1 <= MAX; y1++) {
            if (x1 == 0 && y1 == 0) continue;
            for (int x2 = 0; x2 <= MAX; x2++) {
                for (int y2 = 0; y2 <= MAX; y2++) {
                    if (x2 == 0 && y2 == 0) continue;
                    if (x2 == x1 && y2 == y1) continue;
                    if (is_right(x1, y1, x2, y2)) res++;
                }
            }
        }
    }
    cout << res << " " << res / 2 << endl;
}
