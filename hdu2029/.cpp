#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
string line;
int n;
int main()
{
    scanf("%d", &n);
    while (n--)
    {
        cin >> line;
        string line1 = line;
        reverse(line1.begin(), line1.end());
        line1 == line ? printf("yes\n") : printf("no\n");
    }
    return 0;
}
