#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int main()
{
    vector<int> a = {1, 2, 3, 4, 5, 6, 7, 8, 7, 4, 3, 5, 7, 85, 4};
    vector<int> sol;
    sol.push_back(a.front());
    for (auto i = a.begin() + 1; i != a.end(); i++)
    {
        if (sol.back() <= *i)
        {
            sol.push_back(*i);
        }
        else
        {
            *upper_bound(sol.begin(), sol.end(), *i) = *i;
        }
    }
    for (auto &&i : sol)
    {
        cout << i << " ";
    }
    cout << endl;
    cout << sol.size();
    return 0;
}
