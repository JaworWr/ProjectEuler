#include <iostream>
#include <fstream>
#include <vector>
#include <array>
#include <string>
#include <sstream>
#include <unordered_map>
#include <utility>
#include <cmath>
#include <algorithm>

using namespace std;

vector<string> read_words(const char *path) {
    ifstream f(path);
    stringstream ss;
    ss << f.rdbuf();
    auto str = ss.str();
    
    size_t last_pos = 0, cur_pos = 0;
    vector<string> result;
    while ((cur_pos = str.find(',', cur_pos)) != string::npos) {
        cur_pos++;
        result.push_back(str.substr(last_pos + 1, cur_pos - last_pos - 3));
        last_pos = cur_pos;
    }
    result.push_back(str.substr(last_pos + 1, str.size() - last_pos - 2));
    return result;
}

size_t max_length = 0;

using letter_counts = array<size_t, 26>;

letter_counts get_counts(const string& word) {
    letter_counts counts;
    fill(counts.begin(), counts.end(), 0);
    for (char c : word) {
        counts[c - 'A']++;
    }
    return counts;
}

struct anagram_data {
    letter_counts counts;
    vector<string> anagrams;

    anagram_data() = default;
    anagram_data(const string& word) 
    :counts{get_counts(word)}
    {    
        anagrams.push_back(word);
    }
};

bool is_anagram(const letter_counts& counts, const letter_counts& word_counts) {
    for (int i = 0; i < 26; i++) {
        if (counts[i] != word_counts[i]) {
            return false;
        }
    }
    return true; 
}

using anagram_map = unordered_map<string, anagram_data>;

unordered_map<size_t, vector<string>> by_length;
unordered_map<size_t, anagram_map> anagrams;
vector<pair<string, string>> anagram_pairs;

unordered_map<int, vector<long long>> squares_by_length;

anagram_map get_anagrams(const vector<string>& words) {
    anagram_map result;
    for (const string& word : words) {
        auto word_counts = get_counts(word);
        bool found = false;
        for (auto& p : result) {
            if (is_anagram(p.second.counts, word_counts)) {
                found = true;
                p.second.anagrams.push_back(word);
                break;
            }
        }
        if (!found) {
            result[word] = anagram_data(word);
        }
    }
    return result;
}

vector<int> digits(long long x) {
    vector<int> result;
    while (x > 0) {
        result.push_back(x % 10);
        x /= 10;
    }
    for (size_t i = 0; i < result.size() / 2; i++) {
        swap(result[i], result[result.size() - i - 1]);
    }
    return result;
}

long long from_digits(const vector<int>& x) {
    long long result = 0;
    for (int d : x) {
        result = result * 10 + d;
    }
    return result;
}

long long isqrt(long long x) {
    return (long long) round(sqrt((double) x));
}

vector<int> get_permutation(const string& w1, const string& w2) {
    vector<int> result(w1.length());
    fill(result.begin(), result.end(), -1);
    for (int i = 0; i < (int) w1.length(); i++) {
        int pos = -1;
        do {
            pos = w2.find(w1[i], pos + 1);
        } while(find(result.begin(), result.end(), pos) != result.end());
        result[i] = pos;
    }
    return result;
}

long long check_pair(const string& w1, const vector<int>& permutation) {
    int n = w1.length();
    long long m = -1;
    array<int, 26> c2i;
    array<char, 10> i2c;
    for (long long sq: squares_by_length[n]) {
        fill(i2c.begin(), i2c.end(), -1);
        fill(c2i.begin(), c2i.end(), -1);
        vector<int> dv = digits(sq);
        vector<int> dv_perm(n);
        bool ok = true;
        for (int i = 0; i < n; i++) {
            auto c = w1[i];
            auto d = dv[i];
            if (c2i[c - 'A'] != -1 && c2i[c - 'A'] != d) {
                ok = false;
                break;
            }
            if (i2c[d] != -1 && i2c[d] != c) {
                ok = false;
                break;
            }
            i2c[d] = c;
            c2i[c - 'A'] = d;
            dv_perm[permutation[i]] = dv[i];
        }
        if (!ok || dv_perm[0] == 0) continue;
        auto sq2 = from_digits(dv_perm);
        auto sq2_isqrt = isqrt(sq2);
        if (sq2_isqrt * sq2_isqrt != sq2) continue;
        m = max(m, sq);
        m = max(m, sq2);
    }
    return m;
}

int main() {
    auto words = read_words("p98.txt");
    
    for (const string& word : words) {
        by_length[word.length()].push_back(word);
        max_length = max(max_length, word.length());
    }
    for (const auto& p: by_length) {
        cout << p.first << ": " << p.second.size() << " " << p.second[0] << endl;
    }
    cout << max_length << endl;

    for (size_t l = 2; l <= max_length; l++) {
        anagrams[l] = get_anagrams(by_length.at(l));
    }
    for (const auto& p : anagrams) {
        cout << p.first << ": " << p.second.size() << endl; 
    }
    for (const auto& p : anagrams) {
        for (const auto& p1 : p.second) {
            size_t n = p1.second.anagrams.size();
            for (size_t i = 0; i < n; i++) {
                for (size_t j = i + 1; j < n; j++) {
                    anagram_pairs.push_back({
                        p1.second.anagrams[i],
                        p1.second.anagrams[j]
                    });
                }
            }
        }
    }
    cout << "anagram_pairs: " << anagram_pairs.size() << endl;
    for (size_t i = 0; i < 5; i++) {
        auto& p = anagram_pairs[i];
        cout << p.first << " " << p.second << endl;
    }

    long long i = 1;
    int n_digits = 1;
    while (n_digits <= (long long) max_length) {
        squares_by_length[n_digits].push_back(i * i);
        i++;
        n_digits = (int) log10((double) (i * i)) + 1;
    }
    for (size_t i = 1; i <= max_length; i++) {
        cout << i << ": " 
            << squares_by_length[i].size() << " " 
            << squares_by_length[i][0] << " " 
            << squares_by_length[i].back() << endl;
    }
    cout << from_digits(digits(123045)) << endl;

    auto perm = get_permutation("abcbdef", "cbbadfe");
    for (auto i : perm) {
        cout << i << " ";
    }
    cout << endl;

    // perm = get_permutation("CARE", "RACE");
    // for (auto i : perm) {
    //     cout << i << " ";
    // }
    // cout << endl;
    // auto res = check_pair("CARE", perm);
    // cout << res << endl;

    // ofstream f("pairs.txt", ios::trunc);
    // for (const auto& p : anagram_pairs) {
    //     f << p.first << " " << p.second << endl;
    // }

    long long best = -1;
    string best1, best2;
    for (const auto& p : anagram_pairs) {
        perm = get_permutation(p.first, p.second);
        auto res = check_pair(p.first, perm);
        if (res > best) {
            best = res;
            best1 = p.first;
            best2 = p.second;
        }
    }
    cout << best << " " << best1 << " " << best2 << endl;
}
