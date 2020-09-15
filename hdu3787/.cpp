#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
int main()
{
    string line;
    while (getline(cin, line))
    {
        auto pos = line.find(' ');
        string a = line.substr(0, pos);
        string b = line.substr(pos);
        a.erase(remove(all(a), ','), a.end());
        b.erase(remove(all(b), ','), b.end());
        cout << stoll(a) + stoll(b) << endl;
    }
    return 0;
}
