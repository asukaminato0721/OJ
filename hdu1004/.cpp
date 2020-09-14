#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
int n;
string color;
map<string, int> balloon;
using tp = decltype(balloon)::value_type;
int main()
{
    while (cin >> n && n)
    {
        balloon.clear();
        while (n--)
        {
            cin >> color;
            balloon[color]++;
        }
        cout << max_element(all(balloon), [](const tp &p1, const tp &p2) { return p1.second < p2.second; })->first
             << endl;
    }
    return 0;
}
