#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) (x).begin(), (x).end()
map<char, int> sol;
int main()
{
    int T, n;
    cin >> T;
    while (T--)
    {
        cin >> n;
        sol.clear();
        for (size_t i = 1; i <= n; i++)
        {
            for (auto &&j : to_string(i))
            {
                sol[j]++;
            }
        }
        for (size_t i = 0; i < 9; i++)
        {
            cout << sol[to_string(i)[0]] << " ";
        }
        cout << sol['9'] << endl;
    }
    return 0;
}
