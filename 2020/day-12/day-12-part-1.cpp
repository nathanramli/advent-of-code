#include <iostream>

#define INPUT_OUTPUT

int main()
{
#ifdef INPUT_OUTPUT
    freopen("../input/day-12.txt", "r", stdin);
    freopen("../output/day-12-part-1.txt", "w", stdout);
#endif
    char type;
    int v;
    int step[4] = {0, 0, 0, 0};
    int facing = 90;

    while (std::cin >> type >> v)
    {
        if(type == 'R')
            facing = (facing + v) % 360;
        else if(type == 'L')
            facing = (facing < (v % 360) ? (360 + facing - (v % 360)) : facing - (v % 360)) % 360;
        else if(type == 'F')
            step[(int)(facing / 90)] += v;
        else if(type == 'N')
            step[0] += v;
        else if(type == 'E')
            step[1] += v;
        else if(type == 'S')
            step[2] += v;
        else if(type == 'W')
            step[3] += v;
    }
    printf("%d\n", abs(step[2] - step[0]) + abs(step[1] - step[3]));
}