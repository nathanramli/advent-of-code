#include <iostream>
#include <vector>
#include <string.h>
#include <string>
#include <regex>

#define INPUT_OUTPUT

std::string s;
const int N = 7;
const char e[N][4] = {
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};

bool check(int x, int _val_l, int _val_r)
{
    int temp;
    if (x == 1)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^(19[2-9][0-9]|200[0-2])$"));
    }
    else if (x == 2)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^(201[0-9]|2020)$"));
    }
    else if (x == 3)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^(202[0-9]|2030)$"));
    }
    else if (x == 4)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^(19[0-3]cm|1[5-8][0-9]cm|59in|7[0-6]in|6[0-9]in)$"));
    }
    else if (x == 5)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^#([0-9a-f]{6})$"));
    }
    else if (x == 6)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^(amb|blu|brn|gry|grn|hzl|oth)$"));
    }
    else if (x == 7)
    {
        return std::regex_match(s.begin() + _val_l, s.begin() + _val_r, std::regex("^([0-9]{9})$"));
    }
    return false;
}

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-4.txt", "r", stdin);
    freopen("../output/day-4-part-2.txt", "w", stdout);
#endif

    int ans = 0;

    bool x[N];
    memset(x, false, N);

    while (std::getline(std::cin, s))
    {
        if ((int)s[0] == 0)
        {
            ans++;
            for (size_t i = 0; i < N; i++)
            {
                if (!x[i])
                {
                    ans--;
                    break;
                }
            }
            memset(x, false, N);
        }

        for (size_t i = 0; i < s.size(); i++)
        {
            if (s[i] == ':')
            {
                for (size_t j = 0; j < N; j++)
                {
                    if (strcmp(s.substr(i - 3, 3).c_str(), e[j]) == 0)
                    {
                        if (x[j] == true)
                            break;

                        size_t last = i + 1;
                        for (int k = last; k < s.size(); k++)
                        {
                            if (k == s.size() - 1 || s[k + 1] == ' ')
                            {
                                last = k + 1;
                                break;
                            }
                        }
                        x[j] = check(j + 1, i + 1, last);
                        break;
                    }
                }
            }
        }
    }
    ans++;
    for (size_t i = 0; i < N; i++)
    {
        if (!x[i])
        {
            ans--;
            break;
        }
    }

    printf("%d\n", ans);
}