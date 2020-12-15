#include <iostream>
#include <unordered_map>

#define INPUT_OUTPUT

typedef long long ll;

/**
 * The idea:
 * I make some quick research, the maximum value of last element nor spoken element is never exceeded the total turn
 * So we can use the normal array, instead using a dynamic map.
 * 
 * We generate the array with size N.
 * Where N is the total turn
 */

const int N = 3e7;
int x;
std::pair<int, int> spoken[N];

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-15.txt", "r", stdin);
    freopen("../output/day-15-part-2.txt", "w", stdout);
#endif
    int turn = 1,
        last;
    while (std::cin >> x)
    {
        spoken[x].first = turn++;
        last = x;
    }

    int ans = 0,
        mx = 0;
    for (; turn <= N; turn++)
    {
        if (spoken[last].first == 0)
        {
            spoken[last].first = turn;
            ans = 0;
        }
        else
        {
            if (spoken[last].second != 0)
                ans = spoken[last].first - spoken[last].second;
            else
                ans = 0;

            if (spoken[ans].first != 0)
                spoken[ans].second = spoken[ans].first;
            spoken[ans].first = turn;
        }
        last = ans;
    }
    printf("%d\n", ans);
}