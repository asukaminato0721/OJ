#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int main()
{
    vector<int> a = {1, 2, 3, 4, 5, 6, 7, 8, 7, 4, 3, 5, 7, 85, 4};
    vector<int> sol;
    sol.push_back(a.front());
    size_t maxl = 0;
    for (auto &&i : a)
    {
        if (sol.back() < i)
        {
            sol.push_back(i);
            maxl = max(sol.size(), maxl);
        }
        else
        {
            sol.clear();
            sol.push_back(i);
        }
    }
    cout << maxl;
    return 0;
}
