#include <iostream>
#include <unordered_map>

#define INPUT_OUTPUT

typedef long long ll;

int x;
std::unordered_map<int, std::pair<int, int>> turn_spoken;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-15.txt", "r", stdin);
    freopen("../output/day-15-part-1.txt", "w", stdout);
#endif
    int turn = 1,
        last;
    while (std::cin >> x)
    {
        turn_spoken[x].first = turn++;
        turn_spoken[x].second = -1;
        last = x;
    }

    int ans = 0;
    for (; turn <= 2020; turn++)
    {
        if (turn_spoken.find(last) == turn_spoken.end())
        {
            turn_spoken.insert({last, {turn, -1}});
            ans = 0;
        }
        else
        {
            std::pair<int, int> x = turn_spoken[last];
            if (x.second != -1)
                ans = x.first - x.second;
            else
                ans = 0;
            if (turn_spoken.find(ans) == turn_spoken.end())
                turn_spoken[ans].second = -1;
            else
                turn_spoken[ans].second = turn_spoken[ans].first;
            turn_spoken[ans].first = turn;
        }
        last = ans;
    }
    printf("%d\n", ans);
}