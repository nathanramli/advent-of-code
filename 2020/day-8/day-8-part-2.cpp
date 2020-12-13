#include <iostream>
#include <vector>

#define INPUT_OUTPUT

struct info
{
    int act;
    int val;
};

std::vector<info> arr;
std::vector<bool> visited;
std::vector<bool> last_visited;

struct last
{
    int acc;
    int i;
};

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-8.txt", "r", stdin);
    freopen("../output/day-8-part-2.txt", "w", stdout);
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
        visited.push_back(false);
        arr.push_back(info);
    }

    last last;
    bool change = false;
    int acc = 0;
    int j;
    for (int i = 0; i < arr.size(); i++)
    {
        j = i;
        if (visited[i])
        {
            acc = last.acc;
            i = last.i;
            change = false;
            visited = last_visited;
            continue;
        }
        visited[i] = true;

        if (arr[i].act == 1)
        {
            acc += arr[i].val;
        }
        else if (arr[i].act == -1)
        {
            i += arr[j].val;
            i--;

            if (!change)
            {
                last.i = i;
                last.acc = acc;
                i = j;
                change = true;
                last_visited = visited;
            }
        }
        else
        {
            if (!change)
            {
                last.i = j;
                last.acc = acc;
                i += arr[j].val;
                i--;
                change = true;
                last_visited = visited;
            }
        }
    }

    printf("%d\n", acc);
}