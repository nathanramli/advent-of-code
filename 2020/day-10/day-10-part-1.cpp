#include <iostream>
#include <vector>
#include <algorithm>

#define INPUT_OUTPUT

std::vector<int> q;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-10.txt", "r", stdin);
    freopen("../output/day-10-part-1.txt", "w", stdout);
#endif
    int x;
    while (std::cin >> x)
        q.push_back(x);
    std::sort(q.begin(), q.end());

    x = 0;

    int _1st = 0,
        _3rd = 0;

    size_t len = q.size();
    for (size_t i = 0; i < len; i++)
    {
        if (q[i] - x <= 3 && q[i] - x > 0)
        {
            if (q[i] - x == 1)
                _1st++;
            else if (q[i] - x == 3)
                _3rd++;
            x = q[i];
        }
    }
    if (q[len - 1] == x)
        _3rd++;
    std::cout << _1st * _3rd << '\n';
}