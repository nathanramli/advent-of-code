#include <iostream>
#include <vector>

#define INPUT_OUTPUT

std::string s;
std::vector<std::string> arr;

int main(){
    #ifdef INPUT_OUTPUT
    freopen("../input/day-3.txt", "r", stdin); 
    freopen("../output/day-3-part-1.txt", "w", stdout);
    #endif

    while (std::cin >> s)
        arr.push_back(s);

    size_t len = arr[0].size(),
        v_len = arr.size();

    int x=0, y=0, ans=0;
    while (x+1<v_len)
    {
        x++;
        y = (y + 3) % len;
        if(arr[x][y] == '#')
            ++ans;
    }
    printf("%d\n", ans);
}