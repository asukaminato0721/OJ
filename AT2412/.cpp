#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
vector<int> a;
int main()
{
    int n, k;
    cin >> n >> k;
    int ai;
    for (; n; scanf("%d", &ai), a.push_back(ai), n--)
        ;
    vector<int> b;
    partial_sum(a.begin(), a.end(), back_inserter(b));
    int maxs = *(b.begin() + k - 1);
    for (auto i = b.begin(); i != b.end() - k; i++)
        maxs = max(maxs, *(i + k) - *i);
    cout << maxs << endl;
    return 0;
}
