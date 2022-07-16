#include <iostream>
#include <sstream>
#include <string>
#include <initializer_list>

using namespace std;
using ULL = unsigned long long;

// df[n][m] = # of numbers of length n containing digits marked in the bit mask m
// mask: [0, 1, a]
ULL df[17][8];

int main() {
    // no 0, because numbers can't start with 0
    for (int m : {0b001, 0b010}) {
        df[1][m] = 1;
    }
    // any digit is valid except 0, 1 and a
    df[1][0] = 13;

    
    for (int i = 2; i <= 16; i++) {
        df[i][0] = df[i-1][0] * 13;

        for (int m : {0b001, 0b010, 0b100}) {
            df[i][m] = df[i-1][m] * 14 + df[i-1][0];
        }

        df[i][0b110] = df[i-1][0b110] * 15 + df[i-1][0b100] + df[i-1][0b010];
        df[i][0b101] = df[i-1][0b101] * 15 + df[i-1][0b100] + df[i-1][0b001];
        df[i][0b011] = df[i-1][0b011] * 15 + df[i-1][0b010] + df[i-1][0b001];

        df[i][0b111] = df[i-1][0b111] * 16;
        for (int m : {0b110, 0b101, 0b011}) {
            df[i][0b111] += df[i-1][m];
        }
    }

    cout << df[3][0b111] << endl;
    cout << df[4][0b111] << endl;

    ULL ans = 0;
    for (int i = 3; i <= 16; i++) {
        ans += df[i][0b111];
    }
    cout << ans << endl;

    stringstream ss;
    ss << hex << ans;
    auto s = ss.str();
    for (char& c : s) {
        c = toupper(c);
    }
    cout << s << endl;
}
