#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) (x).begin(), (x).end()
vector<int> sol;
string tmp, line;
int main()
{
    int T;
    cin >> T;
    while (T--)
    {
        sol.clear();
        cin >> line;
        replace(all(line), 'X', ' ');
        stringstream ss(line);
        while (ss >> tmp)
        {
            sol.push_back(tmp.length() * (tmp.length() + 1) / 2);
        }
        cout << accumulate(all(sol), 0) << endl;
    }
    return 0;
}
