#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
#define all(x) (x).begin(), (x).end()
using namespace std;
int cnt = 0;
set<char> sol;
int main()
{
    int n;
    while (cin >> n && n)
    {
        if (cnt)
        {
            cout << endl;
        }
        cnt++;
        bool has_sol = false;
        for (size_t i = 1234; i < 100000 / n + 1; i++)
        {
            sol.clear();
            string tmp = to_string(i);
            if (tmp.size() < 5)
            {
                tmp += "0";
            }
            tmp += to_string(i * n);
            sol.insert(all(tmp));
            if (sol.size() == 10)
            {
                printf("%d / %05d = %d\n", i * n, i, n);
                has_sol = true;
            }
        }
        if (!has_sol)
        {
            printf("There are no solutions for %d.\n", n);
        }
    }
    return 0;
}
