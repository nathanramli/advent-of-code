#include <iostream>
#include <regex>
#include <unordered_map>
#include <queue>

#define INPUT_OUTPUT

std::vector<std::string> arr;
std::string s;
std::queue<int> que;

const int N = 595; // How many variant of bags
int ans = 0;

bool visited[N];
std::vector<int> adj[N];

void bfs(int a)
{
	que.push(a);
	visited[a] = true;

	while(!que.empty())
	{
		int now = que.front();
		que.pop();

		for(int i = 0; i < adj[now].size(); i++)
		{
			int temp = adj[now][i];
			if(!visited[temp])
			{
				que.push(temp);
				visited[temp] = true;
                ans++;
			}
		}
	}
}


int main(){
    #ifdef INPUT_OUTPUT
    freopen("../input/day-7.txt", "r", stdin); 
    freopen("../output/day-7-part-1.txt", "w", stdout);
    #endif

    std::unordered_map<std::string, int> dict;

    int node = 0;
    std::string substr = "";
    int space = 0;

    memset(visited, false, N);

    while (std::getline(std::cin, s)){
        size_t found = s.find("bag");
        int node_a, node_b;
        node_a=node_b=-1;
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
                        else
                            node_b = dict.at(substr);
                    }else{
                        dict.insert({substr, node});
                        if(node_a==-1)
                            node_a=node;
                        else
                            node_b=node;
                        node++;
                    }
                    substr = "";
                    if(node_b!=-1&&node_a!=-1)
                    {
                        adj[node_b].push_back(node_a);
                    }
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
    
    bfs(dict.at("shiny gold"));
    std::cout << ans << '\n';
}