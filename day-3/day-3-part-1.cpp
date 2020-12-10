#include <iostream>
#include <vector>

std::string s;
std::vector<std::string> arr;

int main(){
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