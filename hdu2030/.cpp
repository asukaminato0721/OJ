#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int n;
int main()
{
    scanf("%d", &n);
    getchar();
    while (n--)
    {
        getline(cin, line);
        int s = 0;
        for (auto i = line.begin(); i != line.end(); i++)
            if (*i < 0)
            {
                s++;
                i++;
            }
        cout << s << endl;
    }
    return 0;
}
