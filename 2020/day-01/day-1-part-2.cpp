#include <iostream>
#include <vector>

#define INPUT_OUTPUT

typedef long long ll;
int x;
std::vector<int> arr;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-1.txt", "r", stdin);
    freopen("../output/day-1-part-2.txt", "w", stdout);
#endif

    while (std::cin >> x)
        arr.push_back(x);

    x = 0;
    size_t len = arr.size();
    for (size_t i = 0; i < len && !x; i++)
    {
        for (size_t j = 0; j < len && !x; j++)
        {
            for (size_t k = 0; k < len; k++)
            {
                if (i == j || j == k || k == i)
                    continue;
                if (arr[i] + arr[j] + arr[k] == 2020)
                {
                    std::cout << arr[i] * arr[j] * arr[k] << std::endl;
                    x = 1;
                    break;
                }
            }
        }
    }
}