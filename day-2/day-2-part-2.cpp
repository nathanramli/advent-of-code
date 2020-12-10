#include <iostream>
#include <vector>

typedef long long ll;
int l, h, ans = 0, cnt;
char c, temp;
std::string s;

int main(){
    while (std::cin >> l >> temp >> h >> c >> temp >> s) {
        if(!(s[l - 1] == c) != !(s[h - 1] == c))
        ++ans;
    }
    printf("%d\n", ans);
}