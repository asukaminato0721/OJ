#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
int m, n;
int main()
{
    while (cin >> m >> n)
    {
        cout << m * n / __gcd(m, n) << "\n";
    }
    return 0;
}
