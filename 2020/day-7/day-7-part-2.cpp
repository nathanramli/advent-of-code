#include <iostream>
#include <regex>
#include <unordered_map>
#include <queue>
#include <string.h>

#define INPUT_OUTPUT

std::vector<std::string> arr;
std::string s;
std::queue<int> que;

const int N = 595; // How many variant of bags
int ans = 0;

std::vector<std::pair<int, int>> adj[N];

int dfs(int a)
{
    int sm = 0;

	for(int i = 0; i < adj[a].size(); i++)
	{
		int temp = adj[a][i].first;
        sm += adj[a][i].second * (1 + dfs(temp));
	}
    return sm;
}



int main(){
    #ifdef INPUT_OUTPUT
    freopen("../input/day-7.txt", "r", stdin); 
    freopen("../output/day-7-part-2.txt", "w", stdout);
    #endif

    std::unordered_map<std::string, int> dict;

    int node = 0;
    std::string substr = "";
    int space = 0;

    while (std::getline(std::cin, s)){
        size_t found = s.find("bag");
        int node_a, node_b, cnt;
        node_a=node_b=cnt=-1;
        for(;found != s.npos;){
            for(int i=found-2; i>=0; i--){
                if(i > 0 && s[i-1] == ' ')
                    space++;

                if(i == 0 || space >= 2){
                    substr = s[i] + substr;
                    space = 0;
                    if(dict.find(substr) != dict.end()){                            
                        if(node_a==-1)
                            node_a = dict.at(substr);
                        else{
                            node_b = dict.at(substr);
                            std::string number = "";
                            for(int j=i-2;j>=0;j--){
                                if(s[j] == ' ')
                                    break;
                                number = s[j] + number;
                            }
                            cnt = std::atoi(number.c_str());
                        }
                    }else{
                        dict.insert({substr, node});
                        if(node_a==-1)
                            node_a=node;
                        else{
                            std::string number = "";
                            for(int j=i-2;j>=0;j--){
                                if(s[j] == ' ')
                                    break;
                                number = s[j] + number;
                            }
                            cnt = std::atoi(number.c_str());
                            node_b=node;
                        }
                        node++;
                    }
                    if(node_b!=-1&&node_a!=-1)
                    {
                        adj[node_a].push_back(std::make_pair(node_b, cnt));
                    }
                    substr = "";
                    break;
                }
                else{
                    substr = s[i] + substr;
                }
            }
            found = s.find("bag", found + 3);
        }
        arr.push_back(s);
    }
    
    ans = dfs(dict.at("shiny gold"));
    std::cout << ans << '\n';
}