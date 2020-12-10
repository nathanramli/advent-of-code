#include <iostream>
#include <vector>

typedef long long ll;
int x;
std::vector<int> arr;

int main(){
    while (std::cin >> x)
        arr.push_back(x);
    
    size_t len = arr.size();
    for(size_t i=0;i<len;i++){
        for(size_t j=0; j<len; j++){
            if(i == j) continue;
            if(arr[i] + arr[j] == 2020){
                std::cout << arr[i] * arr[j] << std::endl;
                break;
            }
        }
    }
}