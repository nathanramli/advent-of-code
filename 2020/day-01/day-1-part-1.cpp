#include <iostream>
#include <vector>

#define INPUT_OUTPUT

typedef long long ll;
int x;
std::vector<int> arr;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-01.txt", "r", stdin);
    freopen("../output/day-01-part-1.txt", "w", stdout);
#endif

    while (std::cin >> x)
        arr.push_back(x);

    x = 0;
    size_t len = arr.size();
    for (size_t i = 0; i < len && x == 0; i++)
    {
        for (size_t j = 0; j < len; j++)
        {
            if (i == j)
                continue;
            if (arr[i] + arr[j] == 2020)
            {
                std::cout << arr[i] * arr[j] << std::endl;
                x = 1;
                break;
            }
        }
    }
}