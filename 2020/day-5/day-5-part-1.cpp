#include <iostream>
#include <algorithm>
#include <string.h>
#include <string>
#include <math.h>

#define INPUT_OUTPUT

std::string s;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-5.txt", "r", stdin);
    freopen("../output/day-5-part-1.txt", "w", stdout);
#endif

    int mx = -1;
    while (std::cin >> s)
    {
        int l = 0, r = 127,
            ki = 0, ka = 7;
        for (size_t i = 0; i < s.size(); i++)
        {
            if (i < 7)
            {
                if (s[i] == 'F')
                    r = floor((double)(r + l) / 2.0);
                else
                    l = ceil((double)(r + l) / 2.0);
            }
            else
            {
                if (s[i] == 'L')
                    ka = floor((double)(ki + ka) / 2.0);
                else
                    ki = ceil((double)(ki + ka) / 2.0);
            }
        }
        mx = std::max(mx, l * 8 + ka);
    }
    std::cout << mx << '\n';
}