#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
vector<int64_t> a, b;
int main()
{
    int n;
    cin >> n;
    int ai;
    for (; n; scanf("%d", &ai), a.push_back(ai), n--)
        ;
    partial_sum(a.begin(), a.end(), back_inserter(b));
    for (size_t i = b.size(); i; i--)
    {
        for (auto j = b.begin(); j + i != b.end(); j++)
        {
            if ((*(j + i) - *j) % 7 == 0)
            {
                cout << i << endl;
                return 0;
            }
        }
    }
    return 0;
}
