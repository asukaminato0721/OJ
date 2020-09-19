#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
map<int, vector<int>> pts;
set<int> left_plus_right;
int main()
{
    int T;
    cin >> T;
    int n;
    int x, y;
    while (T--)
    {
        left_plus_right.clear();
        pts.clear();
        for (cin >> n; n; cin >> x >> y, pts[y].push_back(x), n--)
            ;
        for (auto &i : pts)
        {
            auto pt = i.second;
            sort(pt.begin(), pt.end());
            if (pt.size() % 2 == 0)
            {
                for (size_t i = 0; i < pt.size() / 2; i++)
                {
                    left_plus_right.insert(*(pt.begin() + i) + *(pt.end() - i - 1));
                }
            }
            else
            {
                for (size_t i = 0; i < pt.size() / 2 + 1; i++)
                {
                    left_plus_right.insert(*(pt.begin() + i) + *(pt.end() - i - 1));
                }
            }
        }
        if (left_plus_right.size() == 1)
        {
            printf("YES\n");
        }
        else
        {
            printf("NO\n");
        }
    }
    return 0;
}
