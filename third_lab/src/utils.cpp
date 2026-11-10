#include "utils.hpp"
#include <cctype>

using namespace std;

int countSubstr(const string& str, const string& sub) {
    if (sub.empty()) return 0;

    int count = 0;
    size_t pos = 0;

    while ((pos = str.find(sub, pos)) != string::npos) {
        count++;
        pos += sub.length();
    }

    return count;
}

static bool isWordChar(unsigned char c) {
    return isalnum(c) || c >= 0x80;
}

int countWord(const string& str, const string& word) {
    if (word.empty()) return 0;

    int count = 0;
    size_t pos = 0;

    while ((pos = str.find(word, pos)) != string::npos) {
        size_t end = pos + word.length();
        bool leftOk  = pos == 0 || !isWordChar(str[pos - 1]);
        bool rightOk = end == str.length() || !isWordChar(str[end]);

        if (leftOk && rightOk) count++;
        pos = end;
    }

    return count;
}
