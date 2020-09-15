#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
#define all(x) x.begin(), x.end()
int n, m;
int64_t A, B;
int main()
{
    while (scanf("%d", &m) && m)
    {
        scanf("%lld%lld", &A, &B);
        int64_t s = A + B;
        if (s)
        {
            string sol;
            while (s)
            {
                sol = to_string(s % m) + sol;
                s /= m;
            }
            cout << sol << endl;
        }
        else
        {
            printf("0\n");
        }
    }
    return 0;
}
