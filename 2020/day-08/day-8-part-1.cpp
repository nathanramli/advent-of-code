#include <iostream>
#include <vector>

#define INPUT_OUTPUT

struct info
{
    int act;
    int val;
    bool visited;
};

std::vector<info> arr;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-08.txt", "r", stdin);
    freopen("../output/day-08-part-1.txt", "w", stdout);
#endif

    std::string act;
    char op;
    int val;
    while (std::cin >> act >> op >> val)
    {
        info info;
        if (act[0] == 'a')
            info.act = 1;
        else if (act[0] == 'j')
            info.act = -1;
        else
            info.act = 0;

        info.val = op == '+' ? val : -val;
        info.visited = false;
        arr.push_back(info);
    }

    int acc = 0;
    for (int i = 0; i < arr.size(); i++)
    {
        if (arr[i].visited)
            break;

        if (arr[i].act == 1)
            acc += arr[i].val;
        else if (arr[i].act == -1)
        {
            i += arr[i].val;
            i--;
        }

        arr[i].visited = true;
    }

    printf("%d\n", acc);
}