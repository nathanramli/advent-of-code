#include <iostream>
#include <vector>
#include <algorithm>
#include <string.h>
#include <unordered_map>
#include <math.h>

std::string opt, temp, val,
    mask;

typedef long long ll;
std::unordered_map<ll, ll> m;

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
    freopen("../output/day-14-part-2.txt", "w", stdout);
#endif
    int ex = 0;
    while (std::cin >> opt >> temp >> val)
    {
        if (strcmp("mask", opt.c_str()) == 0)
        {
            mask = val;
            for (auto i : mask)
            {
                if (i == 'X')
                    ex++;
            }
        }
        else
        {
            std::string s;
            size_t left = opt.find("[") + 1, right = opt.find("]");
            convertTo36s(opt.substr(left, right - left).c_str(), s);
            std::vector<ll> remainder;
            ll rm = 0;
            for (int i = 0; i < 36; i++)
            {
                if (mask[i] != '0')
                    s[i] = mask[i];

                if (s[i] == 'X')
                    remainder.push_back(35 - i);
                else
                    rm += (s[i] == '1' ? pow(2, (35 - i)) : 0);
            }

            size_t len = remainder.size();
            ll upper = pow(2, len);
            if (len)
            {
                for (int i = 0; i < upper; i++)
                {
                    int k = i;
                    ll x = 0;
                    for (int j = 0; j < len; j++)
                    {
                        if (k % 2)
                            x += pow(2, remainder[j]);
                        k /= 2;
                    }
                    m[x + rm] = std::atoll(val.c_str());
                }
            }
            else
                m[atoll(opt.substr(left, right - left).c_str())] = std::atoll(val.c_str());
        }
    }

    ll sm = 0;
    for (auto i : m)
        sm += i.second;
    std::cout << sm << '\n';
}