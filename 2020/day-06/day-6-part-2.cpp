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
    freopen("../output/day-06-part-2.txt", "w", stdout);
#endif

    int ans = 0;
    bool a[26];
    int idx = 0;
    memset(a, false, 26);

    while (std::getline(std::cin, s))
    {
        if ((int)s[0] == 0)
        {
            idx = 0;
            for (bool i : a)
                if (i)
                    ans++;
            memset(a, false, 26);
            continue;
        }

        bool b[26];
        memset(b, false, 26);

        for (char i : s)
        {
            if (idx == 0)
                a[(int)i - 97] = true;
            else
                b[(int)i - 97] = true;
        }
        if (idx)
            for (int i = 0; i < 26; i++)
            {
                if (!a[i] || !b[i])
                    a[i] = false;
            }
        idx++;
    }
    for (bool i : a)
        if (i)
            ans++;
    printf("%d\n", ans);
}