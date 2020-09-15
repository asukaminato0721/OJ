#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
int n;
int main()
{
    cin >> n;
    while (n--)
    {
        int a, b;
        scanf("%d", &b);
        vector<int64_t> sol;
        while (b--)
        {
            scanf("%d", &a);
            sol.push_back(a);
        }
        long long lcm = sol.front();
        for (auto &&i : sol)
        {
            lcm = lcm * i / __gcd(lcm, i);
        }
        cout << lcm << endl;
    }
    return 0;
}
