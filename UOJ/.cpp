#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
set<pair<int, int>> win = {
    {0, 2}, {0, 3}, {1, 3}, {2, 4}, {3, 4}, {1, 0}, {2, 1}, {3, 2}, {4, 0}, {4, 1}};
queue<int> A, B;
int main()
{
    int n, na, nb;
    int sa = 0, sb = 0;
    scanf("%d%d%d", &n, &na, &nb);
    int tmp;
    for (size_t i = 0; i < na; scanf("%d", &tmp), A.push(tmp), i++)
        ;
    for (size_t i = 0; i < nb; scanf("%d", &tmp), B.push(tmp), i++)
        ;
    while (n--)
    {
        sa += win.count({A.front(), B.front()});
        sb += win.count({B.front(), A.front()});
        A.push(A.front());
        A.pop();
        B.push(B.front());
        B.pop();
    }
    cout << sa << " " << sb;
    return 0;
}
