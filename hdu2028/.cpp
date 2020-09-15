#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int n;
int main()
{
    while (~scanf("%d", &n))
    {
        vector<int64_t> sol;
        while (n--)
        {
            int a;
            scanf("%d", &a);
            sol.push_back(a);
        }
        long long lcm = *sol.begin();
        for (auto &&i : sol)
        {
            lcm = lcm * i / __gcd(lcm, i);
        }
        cout << lcm << endl;
    }
    return 0;
}
