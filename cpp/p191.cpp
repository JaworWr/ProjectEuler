#include <iostream>

using namespace std;
using ULL = unsigned long long;

enum Type {
    LATE = 1, // has been late before
    NO_ABS = 0, // ends with something other than an absence
    ONE_ABS = 2, // ends with 1 absence
    TWO_ABS = 4, // ends with 2 absences
};

// dp[i][m] = # of prize string of length i with conditions m (see neum)
const int N = 30;
ULL dp[N+1][6];

int main() {
    dp[0][NO_ABS] = 1;

    for (int i = 1; i <= N; i++) {
        dp[i][NO_ABS] = dp[i-1][NO_ABS] + dp[i-1][ONE_ABS] + dp[i-1][TWO_ABS];
        dp[i][NO_ABS | LATE] = dp[i-1][NO_ABS | LATE] 
            + dp[i-1][ONE_ABS | LATE] 
            + dp[i-1][TWO_ABS | LATE] 
            + dp[i-1][NO_ABS]
            + dp[i-1][ONE_ABS]
            + dp[i-1][TWO_ABS];
        dp[i][ONE_ABS] = dp[i-1][NO_ABS];
        dp[i][ONE_ABS | LATE] = dp[i-1][NO_ABS | LATE];
        dp[i][TWO_ABS] = dp[i-1][ONE_ABS];
        dp[i][TWO_ABS | LATE] = dp[i-1][ONE_ABS | LATE];
    }

    ULL ans = 0;
    for (int i = 0; i <=5; i++) {
        cout << dp[N][i] << " ";
        ans += dp[N][i];
    }
    cout << endl;
    cout << ans << endl;
}
