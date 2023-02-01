#include <string>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;

string make_str(int len)
{
    string res;
    for (int i = 0; i < len; i++) {
        res.push_back('0');
    }
    return res;
}

bool increment_char(char& c) 
{
    if (c == '9') {
        c = '0';
        return false;
    }
    c++;
    return true;
}

bool increment(string& s)
{
    bool ok = false;
    for (int i = s.length() - 1; i >= 0; i--) {
        ok = increment_char(s[i]);
        if (ok) break;
    }
    return ok;
}

bool check(const string& s1, const string& s2)
{
    int idx = 0;
    for (char c : s1) {
        if (c == s2[idx]) {
            idx++;
            if (idx == s2.length()) {
                return true;
            }
        }
    }
    return false;
}

int matching(const string& s1, const vector<string>& s)
{
    int c = 0;
    for (const string& s2 : s) {
        c += check(s1, s2);
    }
    return c;
}

bool check_all(const string& s1, const vector<string>& s)
{
    return matching(s1, s) == s.size();
}

vector<string> load_strings(const char * path = "p79.txt")
{
    ifstream ifile(path);
    string s;
    vector<string> res;
    while (getline(ifile, s)) {
        res.push_back(s);
    }
    return res;
}

int main()
{
    auto strings = load_strings();
    for (int i = 5; i < 1000; i++) {
        cerr << i << endl;
        auto s = make_str(i);
        if (check_all(s, strings)) {
            cout << s;
            return 0;
        }
        while (increment(s)) {
            if (check_all(s, strings)) {
                cout << s << endl;
                return 0;
            }
        }
    }
}