#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int main()
{
    string s, t;
    while (cin >> t >> s)
    {
        for (auto &&i : s)
        {
            if (i == t.front())
            {
                t = t.substr(1);
            }
            if (t.empty())
            {
                cout << "Yes" << endl;
                break;
            }
        }
        if (!t.empty())
        {
            cout << "No" << endl;
        }
    }
    return 0;
}
