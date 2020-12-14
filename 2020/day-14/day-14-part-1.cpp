#include <iostream>
#include <vector>
#include <algorithm>
#include <string.h>
#include <unordered_map>
#include <math.h>

std::string opt, temp, val,
    mask;

std::unordered_map<int, std::string> m;
typedef long long ll;

void convertTo36s(std::string x, std::string &ret)
{
    ll a = std::atoll(x.c_str());
    for (int i = 35; i >= 0; i--)
    {
        ret = (a % 2 ? '1' : '0') + ret;
        a /= 2;
    }
}

#define INPUT_OUTPUT
int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-14.txt", "r", stdin);
    freopen("../output/day-14-part-1.txt", "w", stdout);
#endif
    while (std::cin >> opt >> temp >> val)
    {
        if (strcmp("mask", opt.c_str()) == 0)
        {
            mask = val;
        }
        else
        {
            std::string s;
            convertTo36s(val, s);
            size_t left = opt.find("[") + 1, right = opt.find("]");
            int key = std::atoi(opt.substr(left, right - left).c_str());
            for (int i = 0; i < 36; i++)
            {
                if (mask[i] != 'X')
                    s[i] = mask[i];
            }
            m[key] = s;
        }
    }

    ll sm = 0, x = 0;
    for (auto i : m)
    {
        int seq = 35;
        for (auto j : i.second)
        {
            sm += (j == '1' ? pow(2, seq) : 0);
            seq--;
        }
    }
    std::cout << sm << '\n';
}