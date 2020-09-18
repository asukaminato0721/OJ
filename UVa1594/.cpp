#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
vector<int> line;
set<vector<int>> repeat;
vector<int> Ducci(vector<int> &line)
{
    vector<int> after;
    line.push_back(line.front());
    adjacent_difference(line.begin(), line.end(), back_inserter(after), [](int i, int j) { return abs(i - j); });
    after.erase(after.begin());
    return after;
}
int main()
{
    int T;
    cin >> T;
    while (T--)
    {
        repeat.clear();
        int n, ai;
        bool end = false;
        line.clear();
        cin >> n;
        for (size_t i = 0; i < n; scanf("%d", &ai), line.push_back(ai), i++)
            ;
        for (size_t i = 0; i < 1000; i++)
        {
            line = Ducci(line);
            if (repeat.count(line)) // 看看是否重复
            {
                printf("LOOP\n");
                end = true;
                break;
            }
            if (all_of(line.begin(), line.end(), [](int i) { return i == 0; })) // 看看是否为 0
            {
                printf("ZERO\n");
                end = true;
                break;
            }
            else
            {
                repeat.insert(line);
            }
        }
        if (!end)
        {
            printf("LOOP\n");
        }
    }
    return 0;
}
/*
4
4
8 11 2 7
5
4 2 0 2 0
7
0 0 0 0 0 0 0
6
1 2 3 1 2 3
*/
