#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
vector<int> a;
int main()
{
    int n;
    int ai;
    cin >> n;
    for (size_t i = 0; i < n; scanf("%d", &ai), a.push_back(ai), i++)
        ;
    vector<int> b;
    partial_sum(a.begin(), a.end(), back_inserter(b));
    for (auto &&i : b)
    {
        cout << i << " ";
    }
    return 0;
}
