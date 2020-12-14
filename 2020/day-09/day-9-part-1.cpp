#include <iostream>
#include <vector>
#include <unordered_map>

#define INPUT_OUTPUT

uint64_t x;
std::vector<uint64_t> q;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-9.txt", "r", stdin);
    freopen("../output/day-9-part-1.txt", "w", stdout);
#endif
    while (std::cin >> x)
        q.push_back(x);

    size_t len = q.size(), lowerbound = 0, preamble = 25;
    for (size_t i = preamble; i < len; i++)
    {
        bool found = false;
        for (size_t j = lowerbound; j < lowerbound + preamble && !found; j++)
        {
            for (size_t k = lowerbound + preamble - 1; k > lowerbound && !found; k--)
            {
                if (j == k)
                    continue;
                if (q[j] + q[k] == q[i])
                    found = true;
            }
        }
        if (found)
            lowerbound++;
        else
        {
            std::cout << q[i] << '\n';
            break;
        }
    }
}