#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int m, n;
string a, b, line;
int main()
{
    scanf("%d", &n);
    getchar();
    for (size_t i = 1; i <= n; i++)
    {
        getline(cin, line);
        auto pos = find(line.begin(), line.end(), ' ');
        a = string(line.begin(), pos);
        b = string(pos + 1, line.end());
        cout << "Case " << i << ":\n"
             << a << " + " << b << " = ";
        auto la = a.size();
        auto lb = b.size();
        if (la > lb)
        {
            b.insert(b.begin(), la - lb, '0');
        }
        else
        {
            a.insert(a.begin(), lb - la, '0');
        }
        bool add1 = false;
        string sol;
        while (!a.empty() && !b.empty())
        {
            auto curr = a.back() - '0' + b.back() - '0';
            sol = to_string((curr + add1) % 10) + sol;
            if (curr + add1 > 9)
            {
                add1 = true;
            }
            else
            {
                add1 = false;
            }
            a.pop_back();
            b.pop_back();
        }
        cout << sol << "\n";
        if (i != n)
        {
            cout << "\n";
        }
    }
    return 0;
}
