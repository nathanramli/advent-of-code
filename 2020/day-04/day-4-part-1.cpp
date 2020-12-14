#include <iostream>
#include <vector>
#include <string.h>

#define INPUT_OUTPUT

std::string s;
const int N = 8;
const char e[N][4] = {
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"};

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-04.txt", "r", stdin);
    freopen("../output/day-04-part-1.txt", "w", stdout);
#endif

    int ans = 0;

    bool x[N];
    memset(x, false, N);

    while (std::getline(std::cin, s))
    {
        if ((int)s[0] == 0)
        {
            ans++;
            for (size_t i = 0; i < N - 1; i++)
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
                        x[j] = true;
                        break;
                    }
                }
            }
        }
    }

    printf("%d\n", ans);
}