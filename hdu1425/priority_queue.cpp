#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
priority_queue<int, vector<int>, greater<int>> sol;
vector<int> solu;
int main()
{
    int n, m;
    while (~scanf("%d%d", &n, &m))
    {
        sol = priority_queue<int, vector<int>, greater<int>>();
        solu.clear();
        while (n--)
        {
            int a;
            scanf("%d", &a);
            sol.push(a);
            while (sol.size() > m)
            {
                sol.pop();
            }
        }
        while (!sol.empty())
        {
            solu.push_back(sol.top());
            sol.pop();
        }
        for (auto i = solu.end() - 1; i > solu.begin(); i--)
        {
            cout << *i << " ";
        }
        cout << *solu.begin() << endl;
    }
    return 0;
}
