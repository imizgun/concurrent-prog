#pragma once
#include <string>

// Количество вхождений подстроки sub в str (без перекрытий)
int countSubstr(const std::string& str, const std::string& sub);

// Количество вхождений слова word в str как отдельного слова
// (не считает "Stoi" внутри "Stoisz")
int countWord(const std::string& str, const std::string& word);
