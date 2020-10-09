#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int n;
string line;
int main()
{
    cin >> n;
    for (size_t i = 0; i < n; i++)
    {
        getchar();
        if (i)
        {
            cout << "\n";
        }
        cin >> line;
        auto tmp = line + line;
        for (size_t T = 1; T <= tmp.size(); T++)
        {
            if (line == tmp.substr(T, line.size()))
            {
                cout << T << "\n";
                break;
            }
        }
    }
    return 0;
}
