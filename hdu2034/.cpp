#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
int n, m;
int main()
{
    while (scanf("%d%d", &n, &m) && n + m > 0)
    {
        set<int> aa, bb;
        int tmp;
        for (size_t i = 0; i < n; i++)
        {
            scanf("%d", &tmp);
            aa.insert(tmp);
        }
        for (size_t i = 0; i < m; i++)
        {
            scanf("%d", &tmp);
            bb.insert(tmp);
        }
        vector<int> c;
        set_difference(all(aa), all(bb), back_inserter(c));
        sort(all(c));
        if (c.empty())
        {
            cout << "NULL";
        }
        else
        {
            for (auto &&i : c)
                cout << i << " ";
        }
        cout << endl;
    }
    return 0;
}
