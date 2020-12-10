#include <iostream>
#include <vector>

typedef long long ll;
int l, h, ans = 0, cnt;
char c, temp;
std::string s;

int main(){
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