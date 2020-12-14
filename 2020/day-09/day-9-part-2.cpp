#include <iostream>
#include <vector>
#include <unordered_map>

#define INPUT_OUTPUT

uint64_t x;
std::vector<uint64_t> q;

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-09.txt", "r", stdin);
    freopen("../output/day-09-part-2.txt", "w", stdout);
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
        if(found)
            lowerbound++;
        else{
            for(size_t l = 0; l < i - 1 && !found; l++){
                uint64_t sm = q[l];
                uint64_t mn = q[l], mx = q[l];
                for(size_t m = l + 1; m < i && !found; m++){
                    sm += q[m];
                    mn = std::min(q[m], mn);
                    mx = std::max(q[m], mx);
                    if(sm == q[i]){
                        std::cout << mx + mn << '\n';
                        found=true;
                    }
                }
            }
            if(found)
                break;
        }
    }
}