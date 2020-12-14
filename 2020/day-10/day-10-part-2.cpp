#include <iostream>
#include <vector>
#include <algorithm>
#include <memory.h>

#define INPUT_OUTPUT

std::vector<int> q;
std::vector<int> adj[200];
uint64_t dp[200];

uint64_t ans = 0;
int x,
    target = 0;

uint64_t dfs(int a)
{
    uint64_t sm = 0;
    if (a == target)
    {
        return 1;
    }
    else
    {
        for (int i = 0; i < adj[a].size(); i++)
        {
            if (dp[adj[a][i]])
            {
                sm += dp[adj[a][i]];
                continue;
            }
            uint64_t temp = dfs(adj[a][i]);
            sm += temp;
            dp[adj[a][i]] = temp;
        }
    }
    return sm;
}

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-10.txt", "r", stdin);
    freopen("../output/day-10-part-2.txt", "w", stdout);
#endif
    while (std::cin >> x)
    {
        q.push_back(x);
        target = std::max(target, x);
    }
    std::sort(q.begin(), q.end());

    x = 0;
    size_t len = q.size();
    for (size_t j = 0; j < len; j++)
    {
        for (size_t l = j; l < len; l++)
        {
            if (q[l] - x <= 3 && q[l] - x > 0)
            {
                adj[x].push_back(q[l]);
            }
            else if (q[l] - x > 3)
                break;
        }
        x = q[j];
    }
    memset(dp, 0, 200);
    std::cout << dfs(0) << '\n';
}