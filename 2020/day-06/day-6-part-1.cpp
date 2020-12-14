#include <iostream>
#include <vector>
#include <string.h>
#include <string>

#define INPUT_OUTPUT

std::string s;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-06.txt", "r", stdin);
    freopen("../output/day-06-part-1.txt", "w", stdout);
#endif

    int ans = 0;
    bool a[26];
    memset(a, false, 26);

    while (std::getline(std::cin, s))
    {
        if ((int)s[0] == 0)
        {
            for (bool i : a)
                if (i)
                    ans++;
            memset(a, false, 26);
        }

        for (char i : s)
            a[(int)i - 97] = true;
    }
    for (bool i : a)
        if (i)
            ans++;
    printf("%d\n", ans);
}