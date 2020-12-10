#include <iostream>
#include <vector>

#define INPUT_OUTPUT

typedef long long ll;
int l, h, ans = 0, cnt;
char c, temp;
std::string s;

int main(){
    #ifdef INPUT_OUTPUT
    freopen("../input/day-2.txt", "r", stdin); 
    freopen("../output/day-2-part-1.txt", "w", stdout);
    #endif

    while (std::cin >> l >> temp >> h >> c >> temp >> s)
    {
        cnt = 0;
        for(char i : s) if(i == c) 
            cnt++;
        if(cnt >= l && cnt <= h)
            ++ans;
    }
    printf("%d\n", ans);
}