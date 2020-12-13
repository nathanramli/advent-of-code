#include <iostream>
#include <vector>

#define INPUT_OUTPUT

typedef long long ll;
int l, h, ans = 0, cnt;
char c, temp;
std::string s;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-2.txt", "r", stdin);
    freopen("../output/day-2-part-2.txt", "w", stdout);
#endif

    while (std::cin >> l >> temp >> h >> c >> temp >> s)
    {
        if (!(s[l - 1] == c) != !(s[h - 1] == c))
            ++ans;
    }
    printf("%d\n", ans);
}