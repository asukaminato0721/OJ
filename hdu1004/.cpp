#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int n;
string color;
int main()
{
    while (cin >> n && n)
    {
        map<string, int> balloon;
        while (n--)
        {
            cin >> color;
            balloon[color]++;
        }
        string maxcolor;
        int maxnum = 0;
        for (auto &&i : balloon)
        {
            if (maxnum < i.second)
            {
                maxnum = i.second;
                maxcolor = i.first;
            }
        }
        cout << maxcolor << endl;
    }
    return 0;
}
