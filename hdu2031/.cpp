#pragma GCC optimize(3, "Ofast", "inline")
#include <bits/stdc++.h>
using namespace std;
map<int, char> num = {
    {0, '0'},
    {1, '1'},
    {2, '2'},
    {3, '3'},
    {4, '4'},
    {5, '5'},
    {6, '6'},
    {7, '7'},
    {8, '8'},
    {9, '9'},
    {10, 'A'},
    {11, 'B'},
    {12, 'C'},
    {13, 'D'},
    {14, 'E'},
    {15, 'F'}};
int main()
{
    int a, b;
    while (~scanf("%d%d", &a, &b))
    {
        string sol;
        bool neg = (a < 0);
        if (a < 0)
        {
            a = -a;
        }
        while (a)
        {
            sol = num[a % b] + sol;
            a /= b;
        }
        if (neg)
        {
            sol = '-' + sol;
        }
        cout << sol << "\n";
    }
    return 0;
}
